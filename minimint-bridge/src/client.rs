//! Minimint client with simpler types
use anyhow::anyhow;
use bitcoin::{hashes::sha256, psbt::Error};
use fedimint_api::{
    db::{Database, DatabaseKeyPrefixConst},
    encoding::{Decodable, Encodable},
    module::ApiError,
    NumPeers,
};
use fedimint_core::modules::ln::contracts::ContractId;
use fedimint_core::{config::ClientConfig, modules::ln::contracts::IdentifyableContract};
use futures::{stream::FuturesUnordered, StreamExt};
use lightning_invoice::Invoice;
use mint_client::{api::WsFederationApi, UserClient, UserClientConfig};
use mint_client::{api::WsFederationConnect, query::CurrentConsensus};
use std::fmt;
use std::time::{Duration, SystemTime};

use crate::payments::{Payment, PaymentDirection, PaymentKey, PaymentKeyPrefix, PaymentStatus};

pub struct Client {
    pub label: String,
    pub connection_status: ConnectionStatus,
    pub cfg_json: String,
    pub(crate) client: Option<UserClient>,
}

#[derive(Clone, Debug, Encodable, Decodable, PartialEq)]
pub enum ConnectionStatus {
    NotConnected,
    Connected,
    NotConfigured,
}
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl Client {
    pub fn fetch_payment(&self, payment_hash: &sha256::Hash) -> Option<Payment> {
        match self.client.as_ref() {
            Some(client) => client
                .db()
                .get_value(&PaymentKey(payment_hash.clone()))
                .expect("Db error"),
            None => None,
        }
    }

    pub fn list_payments(&self) -> Vec<Payment> {
        match self.client.as_ref() {
            Some(client) => client
                .db()
                .find_by_prefix(&PaymentKeyPrefix)
                .map(|res| res.expect("Db error").1)
                .collect(),
            None => vec![],
        }
    }

    pub fn save_payment(&self, payment: &Payment) {
        match self.client.as_ref() {
            Some(client) => client
                .db()
                .insert_entry(&PaymentKey(payment.invoice.payment_hash().clone()), payment)
                .expect("Db error"),
            None => None,
        };
    }

    pub fn update_payment_status(&self, payment_hash: &sha256::Hash, status: PaymentStatus) {
        if let Some(mut payment) = self.fetch_payment(&payment_hash) {
            payment.status = status;
            match self.client.as_ref() {
                Some(client) => client
                    .db()
                    .insert_entry(&PaymentKey(*payment_hash), &payment)
                    .expect("Db error"),
                None => None,
            };
        }
        // TODO: what to do if this payment doesn't exist?
    }

    pub fn save_user_data(&self, user_data: &str) {
        match self.client.as_ref() {
            Some(client) => client
                .db()
                .insert_entry(&UserDataKey, &user_data.to_string())
                .expect("Db error"),
            None => None,
        };
    }

    pub fn fetch_user_data(&self) -> Option<String> {
        match self.client.as_ref() {
            Some(client) => match client.db().get_value(&UserDataKey) {
                Ok(v) => match v {
                    Some(user_data) => Some(user_data),
                    None => return None,
                },
                Err(_) => return None,
            },
            None => return None,
        }
    }
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ConfigKey;
#[derive(Debug, Clone, Encodable, Decodable)]
pub struct UserDataKey;
const CONFIG_KEY_PREFIX: u8 = 0x50;
const USER_DATA_KEY_PREFIX: u8 = 0x60;

impl DatabaseKeyPrefixConst for ConfigKey {
    const DB_PREFIX: u8 = CONFIG_KEY_PREFIX;
    type Key = Self;

    type Value = String;
}

impl DatabaseKeyPrefixConst for UserDataKey {
    const DB_PREFIX: u8 = USER_DATA_KEY_PREFIX;
    type Key = Self;

    type Value = String;
}

impl Client {
    pub async fn try_load(db: Database, label: &str) -> anyhow::Result<Self> {
        if let Some(cfg_json) = db.get_value(&ConfigKey).expect("db error") {
            return Ok(Self::new(db, &cfg_json, label).await?);
        }
        Err(anyhow!("no config string"))
    }

    pub async fn new(db: Database, cfg_json: &str, label: &str) -> anyhow::Result<Self> {
        let connect_cfg: WsFederationConnect = serde_json::from_str(cfg_json)?;
        let api = WsFederationApi::new(connect_cfg.members);
        let cfg_result: Result<ClientConfig, mint_client::api::ApiError> = api
            // FIXME: is this the correct policy?
            .request(
                "/config",
                (),
                CurrentConsensus::new(api.peers().one_honest()),
            )
            .await;

        if cfg_result.is_ok() {
            db.insert_entry(&ConfigKey, &cfg_json.to_string())
                .expect("db error");

            Ok(Self {
                label: String::from(label),
                connection_status: ConnectionStatus::Connected,
                cfg_json: String::from(cfg_json),
                client: Some(UserClient::new(
                    UserClientConfig(cfg_result.unwrap().clone()),
                    db,
                    Default::default(),
                )),
            })
        } else {
            Ok(Self {
                label: String::from(label),
                connection_status: ConnectionStatus::NotConnected,
                cfg_json: String::from(cfg_json),
                client: None,
            })
        }
        // FIXME: this isn't the right thing to store
    }

    pub fn federation_name(&self) -> Option<String> {
        if let Some(client) = self.client.as_ref() {
            return Some(client.config().0.federation_name);
        }
        return None;
    }

    pub fn balance(&self) -> u64 {
        if let Some(client) = self.client.as_ref() {
            return ((client.coins().total_amount().milli_sat as f64) / 1000.) as u64;
        }
        return 0;
    }

    pub fn network(&self) -> Option<bitcoin::Network> {
        if let Some(client) = self.client.as_ref() {
            return Some(client.wallet_client().config.network);
        }
        return None;
    }

    async fn pay_inner(&self, bolt11: Invoice) -> anyhow::Result<()> {
        match self.client.as_ref() {
            Some(client) => {
                let mut rng = rand::rngs::OsRng::new().unwrap();
                let (contract_id, outpoint) = client
                    .fund_outgoing_ln_contract(bolt11.clone(), &mut rng)
                    .await?;

                client.await_outgoing_contract_acceptance(outpoint).await?;

                let result = client
                    .await_outgoing_contract_execution(contract_id, &mut rng)
                    .await;

                // FIXME: actually check that a refund happened
                if result.is_err() {
                    client.fetch_all_coins().await;
                }

                return Ok(result?);
            }
            None => return Err(anyhow!("Client not connected")),
        };
    }

    // FIXME: this won't let you attempt to pay an invoice where previous payment failed
    // Trying to avoid losing funds at the expense of UX ...
    pub fn can_pay(&self, invoice: &Invoice) -> bool {
        // If there isn't an outgoing fluttermint payment, we can pay
        self.list_payments()
            .iter()
            .filter(|payment| payment.outgoing() && &payment.invoice == invoice)
            .next()
            .is_none()
    }

    pub async fn pay(&self, bolt11: String) -> anyhow::Result<()> {
        let invoice: Invoice = bolt11.parse()?;

        if !self.can_pay(&invoice) {
            return Err(anyhow!("Can't pay invoice twice"));
        }

        match self.client.as_ref() {
            Some(client) => match self.pay_inner(invoice.clone()).await {
                Ok(_) => {
                    self.save_payment(&Payment::new(
                        invoice,
                        PaymentStatus::Paid,
                        PaymentDirection::Outgoing,
                    ));
                    client.fetch_all_coins().await;
                    Ok(())
                }
                Err(e) => {
                    self.save_payment(&Payment::new(
                        invoice,
                        PaymentStatus::Failed,
                        PaymentDirection::Outgoing,
                    ));
                    Err(e)
                }
            },
            None => Err(anyhow!("client not connected")),
        }
    }

    pub async fn invoice(&self, amount: u64, description: String) -> anyhow::Result<String> {
        let mut rng = rand::rngs::OsRng::new().unwrap();

        let amt = fedimint_api::Amount::from_sat(amount);
        match self.client.as_ref() {
            Some(client) => {
                let confirmed_invoice = client
                    .generate_invoice(amt, description, &mut rng)
                    .await
                    .expect("Couldn't create invoice");
                let invoice = confirmed_invoice.invoice;

                // Save the keys and invoice for later polling`
                self.save_payment(&Payment::new(
                    invoice.clone(),
                    PaymentStatus::Pending,
                    PaymentDirection::Incoming,
                ));
                tracing::info!("saved invoice to db");

                Ok(invoice.to_string())
            }
            None => Err(anyhow!("client not connected")),
        }
    }

    // FIXME: there should be a cheaper way to check if we're connected
    pub async fn check_connection(&self) -> bool {
        match self.client.as_ref() {
            Some(client) => match client.fetch_registered_gateways().await {
                Ok(_) => true,
                Err(_) => false,
            },
            None => false,
        }
    }

    async fn block_height(&self) -> anyhow::Result<u64> {
        match self.client.as_ref() {
            Some(client) => Ok(client
                .wallet_client()
                .context
                .api
                .fetch_consensus_block_height()
                .await?),
            None => Err(anyhow!("client not connected")),
        }
    }

    pub async fn poll(&self) {
        // TODO.... try to reconnect client if connection status is NotConnected
        let mut last_outgoing_check = SystemTime::now();
        loop {
            // Try to complete incoming payments
            match self.client.as_ref() {
                Some(client) => {
                    let mut requests = self
                        .list_payments()
                        .into_iter()
                        // TODO: should we filter
                        .filter(|payment| {
                            !payment.paid() && !payment.expired() && payment.incoming()
                        })
                        .map(|payment| async move {
                            // FIXME: don't create rng in here ...
                            let invoice_expired = payment.invoice.is_expired();
                            let rng = rand::rngs::OsRng::new().unwrap();
                            let payment_hash = payment.invoice.payment_hash();
                            tracing::debug!("fetching incoming contract {:?}", &payment_hash);
                            let result = client
                                .claim_incoming_contract(
                                    ContractId::from_hash(payment_hash.clone()),
                                    rng.clone(),
                                )
                                .await;
                            if let Err(_) = result {
                                tracing::debug!("couldn't complete payment: {:?}", &payment_hash);
                                // Mark it "expired" in db if we couldn't claim it and invoice is expired
                                if invoice_expired {
                                    self.update_payment_status(
                                        payment_hash,
                                        PaymentStatus::Expired,
                                    );
                                }
                            } else {
                                tracing::debug!("completed payment: {:?}", &payment_hash);
                                self.update_payment_status(payment_hash, PaymentStatus::Paid);
                                client.fetch_all_coins().await;
                            }
                        })
                        .collect::<FuturesUnordered<_>>();

                    // FIXME: is there a better way to consume these futures?
                    while let Some(_) = requests.next().await {
                        tracing::info!("completed api request");
                    }

                    // Only check outgoing payments once per minute
                    if last_outgoing_check
                        .elapsed()
                        .expect("Unix time not available")
                        > Duration::from_secs(60)
                    {
                        // Try to complete outgoing payments
                        let consensus_block_height = match self.block_height().await {
                            Ok(height) => height,
                            Err(_) => {
                                tracing::error!("failed to get block height");
                                continue;
                            }
                        };

                        // TODO: only do this once per minute
                        tracing::info!("looking for refunds...");
                        let mut requests = client
                            .ln_client()
                            .refundable_outgoing_contracts(consensus_block_height)
                            .into_iter()
                            .map(|contract| async move {
                                tracing::info!(
                                    "attempting to get refund {:?}",
                                    contract.contract_account.contract.contract_id(),
                                );
                                match client
                                    .try_refund_outgoing_contract(
                                        contract.contract_account.contract.contract_id(),
                                        rand::rngs::OsRng::new().unwrap(),
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        tracing::info!("got refund");
                                        client.fetch_all_coins().await;
                                    }
                                    Err(e) => tracing::info!("refund failed {:?}", e),
                                }
                            })
                            .collect::<FuturesUnordered<_>>();

                        // FIXME: is there a better way to consume these futures?
                        while let Some(_) = requests.next().await {
                            tracing::info!("completed api request");
                        }
                        last_outgoing_check = SystemTime::now();
                    }

                    fedimint_api::task::sleep(std::time::Duration::from_secs(1)).await;
                }
                None => {
                    fedimint_api::task::sleep(std::time::Duration::from_secs(1)).await;
                    // TODO: reconnect code here
                }
            }
        }
    }
}
