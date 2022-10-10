use std::path::Path;
use std::sync::Arc;

use bitcoin::hashes::sha256;
use bitcoin::Network;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use lightning_invoice::{Invoice, InvoiceDescription};
use mint_client::utils::network_to_currency;
use tokio::runtime;
use tokio::sync::Mutex;

use crate::client::ConnectionStatus;
use crate::client_manager::ClientManager;
use crate::database::{SledDatabase, SledTree};
use crate::init_tracing;
use crate::payments::{PaymentDirection, PaymentStatus};

static CLIENT_DB_FILENAME: &str = "client.db";
static DEFAULT_TREE_ID: &str = "__sled__default";

lazy_static! {
    static ref RUNTIME: runtime::Runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build runtime");
}

static GLOBAL_API: Mutex<Option<API>> = Mutex::const_new(None);

struct API {
    client_manager: ClientManager,
    db: Arc<Mutex<SledDatabase>>,
    db_default_tree: Mutex<SledTree>,
}

pub fn init(path: String) {
    init_tracing();
    RUNTIME.block_on(async {
        if let Ok(db) = SledDatabase::open(Path::new(&path).join(CLIENT_DB_FILENAME)) {
            let db_arc = Arc::new(Mutex::new(db));
            if let Ok(tree) = db_arc.clone().lock().await.open_tree(DEFAULT_TREE_ID) {
                *GLOBAL_API.lock().await = Some(API {
                    db: db_arc.clone(),
                    db_default_tree: Mutex::new(tree),
                    client_manager: ClientManager::new(db_arc.clone()),
                })
            }
        }
    });

    // TODO initial loading of dbs:
    // load db "labels.db"
    // load all "<label>.db" files and initialise clients

    /*
    RUNTIME.block_on(async {
        if global_client::is_some().await {
            return connection_status_private().await;
        };
        let filename = Path::new(&path).join("client.db");
        // TODO: use federation name as "tree"
        let db = SledDb::open(filename, "client")?;
        if let Some(client) = Client::try_load(db.into()).await? {
            let client = Arc::new(client);
            global_client::set(client.clone()).await;
            let status = connection_status_private().await?;
            return Ok(status);
        }
        Ok(ConnectionStatus::NotConfigured)
    })
    */
}

/*
pub async fn delete_database(&self) -> Result<()> {
  // Wipe database COMPLETELY!!
  // EVERYTHING IS GONE!!
  if let Some(user_dir) = self.user_dir.lock().await.as_ref() {
      std::fs::remove_dir_all(Path::new(&user_dir).join(CLIENT_DB_FILENAME))?;
  }
  Ok(())
}
 */
/// Bridge representation of a fedimint node
#[derive(Clone, Debug)]
pub struct BridgeClientInfo {
    pub label: String, // unique label of the client
    pub balance: u64,  // balance in satoshis
    pub federation_name: String,
    pub user_data: String, // data which can be set, but is only useful to the user
}

pub fn get_client(label: String) -> Result<BridgeClientInfo> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let client_result = api.client_manager.get_client_by_label(label.as_str()).await;

            if client_result.is_err() {
                return Err(anyhow!("no such client"));
            }

            let client = client_result.unwrap().clone();

            return Ok(BridgeClientInfo {
                label: label.clone(),
                balance: client.balance(),
                federation_name: client.federation_name(),
                user_data: client.fetch_user_data(),
            });
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn get_clients() -> Result<Vec<BridgeClientInfo>> {
    RUNTIME.block_on(async {
        let mut r = Vec::new();
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let client_labels = api.client_manager.get_client_labels().await;

            for client_label in client_labels.iter() {
                let client_result = api
                    .client_manager
                    .get_client_by_label(client_label.as_str())
                    .await;

                if client_result.is_ok() {
                    let client = client_result.unwrap().clone();
                    r.push(BridgeClientInfo {
                        label: client_label.clone(),
                        balance: client.balance(),
                        federation_name: client.federation_name(),
                        user_data: client.fetch_user_data(),
                    })
                }
            }
            return Ok(r);
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn join_federation(config_url: String) -> Result<BridgeClientInfo> {
    // TODO: throw error when federation was already joined
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let client = api.client_manager.add_client(config_url.as_str()).await?;
            return Ok(BridgeClientInfo {
                label: client.label.clone(),
                balance: client.balance(),
                federation_name: client.federation_name(),
                user_data: client.fetch_user_data(),
            });
        }
        Err(anyhow!("api not configured"))
    })
}

/// Unset client and wipe database. Ecash will be destroyed. Use with caution!!!
pub fn leave_federation(label: String) -> Result<()> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            api.client_manager.remove_client(label.as_str()).await?;
            api.client_manager
                .delete_client_database(label.as_str())
                .await?;
            return Ok(());
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn balance(label: String) -> Result<u64> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            return Ok(api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?
                .balance());
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn pay(label: String, bolt11: String) -> Result<()> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            return api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?
                .pay(bolt11)
                .await;
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn invoice(label: String, amount: u64, description: String) -> Result<String> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let client = api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?;

            if client.network() == Network::Bitcoin && amount > 60000 {
                return Err(anyhow!("Maximum invoice size on mainnet is 60000 sats"));
            }

            return client.invoice(amount, description).await;
        }
        Err(anyhow!("api not configured"))
    })
}

// TODO: impl From<Payment>
// Do the "expired" conversion in there, too
#[derive(Clone, Debug)]
pub struct BridgePayment {
    pub invoice: BridgeInvoice,
    pub status: PaymentStatus,
    pub created_at: u64,
    pub paid: bool,
    pub direction: PaymentDirection,
}

#[derive(Clone, Debug)]
pub struct BridgeInvoice {
    pub payment_hash: String,
    pub amount: u64,
    pub description: String,
    pub invoice: String,
}

pub fn fetch_payment(label: String, payment_hash: String) -> Result<BridgePayment> {
    let hash: sha256::Hash = payment_hash.parse()?;
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let payment = api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?
                .fetch_payment(&hash)
                .ok_or(anyhow!("payment not found"))?;
            return Ok(BridgePayment {
                invoice: decode_invoice_inner(&payment.invoice)?,
                status: payment.status,
                created_at: payment.created_at,
                paid: payment.paid(),
                direction: payment.direction,
            });
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn list_payments(label: String) -> Result<Vec<BridgePayment>> {
    println!("Listing payments...");
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let payments = api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?
                .list_payments()
                .iter()
                // TODO From impl
                .map(|payment| BridgePayment {
                    // FIXME: don't expect
                    invoice: decode_invoice_inner(&payment.invoice)
                        .expect("couldn't decode invoice"),
                    status: payment.status,
                    created_at: payment.created_at,
                    paid: payment.paid(),
                    direction: payment.direction,
                })
                .collect();
            return Ok(payments);
        }
        Err(anyhow!("api not configured"))
    })
}

async fn configured_status_private(label: &str) -> Result<bool> {
    if let Some(api) = GLOBAL_API.lock().await.as_ref() {
        return Ok(api.client_manager.client_exists(label).await);
    }
    Err(anyhow!("api not configured"))
}

pub fn configured_status(label: String) -> Result<bool> {
    RUNTIME.block_on(async { configured_status_private(label.as_str()).await })
}

async fn connection_status_private(label: &str) -> Result<ConnectionStatus> {
    if let Some(api) = GLOBAL_API.lock().await.as_ref() {
        if !api.client_manager.client_exists(label).await {
            return Ok(ConnectionStatus::NotConfigured);
        }
        return match api
            .client_manager
            .get_client_by_label(label)
            .await?
            .check_connection()
            .await
        {
            true => Ok(ConnectionStatus::Connected),
            false => Ok(ConnectionStatus::NotConnected),
        };
    }
    Err(anyhow!("api not configured"))
}

pub fn connection_status(label: String) -> Result<ConnectionStatus> {
    RUNTIME.block_on(async { connection_status_private(label.as_str()).await })
}

pub fn network(label: String) -> Result<String> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            return Ok(api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?
                .network()
                .to_string());
        }
        Err(anyhow!("api not configured"))
    })
}

pub fn calculate_fee(bolt11: String) -> Result<Option<u64>> {
    let invoice: Invoice = bolt11.parse()?;
    let fee = invoice
        .amount_milli_satoshis()
        .map(|msat| {
            // Add 1% fee margin
            msat / 100
        })
        // FIXME janky msat -> sat conversion
        .map(|msat| (msat as f64 / 1000 as f64).round() as u64);
    Ok(fee)
}

/// Bridge representation of a fedimint node
#[derive(Clone, Debug)]
pub struct BridgeGuardianInfo {
    pub name: String,
    pub address: String,
    pub online: bool,
}

/// Bridge representation of a fedimint node
#[derive(Clone, Debug)]
pub struct BridgeFederationInfo {
    pub name: String,
    pub network: String,
    pub current: bool,
    pub guardians: Vec<BridgeGuardianInfo>,
}

/// Returns the federations we're members of
///
/// At most one will be `active`
pub fn list_federations() -> Vec<BridgeFederationInfo> {
    return vec![
        BridgeFederationInfo {
            name: "Trimont State Bank".into(),
            network: Network::Bitcoin.to_string(),
            current: true,
            guardians: vec![
                BridgeGuardianInfo {
                    name: "Tony".into(),
                    address: "https://locahost:5000".into(),
                    online: true,
                },
                BridgeGuardianInfo {
                    name: "Cal".into(),
                    address: "https://locahost:6000".into(),
                    online: false,
                },
            ],
        },
        BridgeFederationInfo {
            name: "CypherU".into(),
            network: Network::Signet.to_string(),
            current: false,
            guardians: vec![
                BridgeGuardianInfo {
                    name: "Eric".into(),
                    address: "https://locahost:7000".into(),
                    online: false,
                },
                BridgeGuardianInfo {
                    name: "Obi".into(),
                    address: "https://locahost:8000".into(),
                    online: true,
                },
            ],
        },
    ];
}

/// Switch to a federation that we've already joined
///
/// This assumes federation config is already saved locally
pub fn switch_federation(_federation: BridgeFederationInfo) -> Result<()> {
    Ok(())
}

/// Decodes an invoice and checks that we can pay it
pub fn decode_invoice(label: String, bolt11: String) -> Result<BridgeInvoice> {
    RUNTIME.block_on(async {
        if let Some(api) = GLOBAL_API.lock().await.as_ref() {
            let client = api
                .client_manager
                .get_client_by_label(label.as_str())
                .await?;
            let invoice: Invoice = match bolt11.parse() {
                Ok(i) => Ok(i),
                Err(_) => Err(anyhow!("Invalid lightning invoice")),
            }?;
            if !client.can_pay(&invoice) {
                return Err(anyhow!("Can't pay invoice twice"));
            }
            if network_to_currency(client.network()) != invoice.currency() {
                return Err(anyhow!(format!(
                    "Wrong network. Expected {}, got {}",
                    network_to_currency(client.network()),
                    invoice.currency()
                )));
            }
            if invoice.is_expired() {
                return Err(anyhow!("Invoice is expired"));
            }
            return decode_invoice_inner(&invoice);
        }
        Err(anyhow!("api not configured"))
    })
}

fn decode_invoice_inner(invoice: &Invoice) -> anyhow::Result<BridgeInvoice> {
    let amount = invoice
        .amount_milli_satoshis()
        // FIXME:justin this is janky
        .map(|amount| (amount as f64 / 1000 as f64).round() as u64)
        .ok_or(anyhow!("Invoice missing amount"))?;

    // We might get no description
    let description = match invoice.description() {
        InvoiceDescription::Direct(desc) => desc.to_string(),
        InvoiceDescription::Hash(_) => "".to_string(),
    };

    Ok(BridgeInvoice {
        amount,
        description,
        invoice: invoice.to_string(),
        payment_hash: invoice.payment_hash().to_string(),
    })
}

#[cfg(test)]
mod tests {}
