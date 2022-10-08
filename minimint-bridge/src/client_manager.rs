use crate::client::Client;
use anyhow::{anyhow, Result};
use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::sha256::Hash;
use fedimint_api::BitcoinHash;
use fedimint_api::{
    db::{Database, DatabaseKeyPrefixConst},
    encoding::{Decodable, Encodable},
    NumPeers,
};
use fedimint_sled::SledDb;
use lazy_static::lazy_static;
use mint_client::api::WsFederationConnect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
extern crate lazy_static;

lazy_static! {
    static ref GLOBAL_CLIENTS: Mutex<HashMap<String, Mutex<Arc<Client>>>> = {
        return Mutex::new(HashMap::new());
    };
}
static GLOBAL_POLLER: Mutex<Option<JoinHandle<()>>> = Mutex::const_new(None);
static GLOBAL_USER_DIR: Mutex<Option<String>> = Mutex::const_new(None);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientsState {
    pub active_clients: Vec<String>,
}

// Justin: static function to return a reference to the federation you're working on.
// Dart side can call methods on it.

pub async fn init(path: String) -> Result<()> {
    set_user_dir(path.clone()).await;
    init_clients().await?;
    Ok(())
}

async fn init_clients() -> Result<()> {
    let clients_state_result = load_clients_state().await;
    let path = get_user_dir().await?;
    if clients_state_result.is_ok() {
        for client_label in clients_state_result.unwrap().active_clients.iter() {
            let client_db_filename = Path::new(&path).join(client_label);
            let db = SledDb::open(client_db_filename, "client")?;

            if let Some(client) = Client::try_load(db.into(), client_label.as_str()).await? {
                let client = Arc::new(client);

                GLOBAL_CLIENTS.lock().await.insert(
                    String::from(client_label.as_str()),
                    Mutex::new(client.clone()),
                );
                tracing::info!("loading client {}", client_label);
            } else {
                tracing::info!("no database for client {}", client_label);
            }
        }
    }
    Ok(())
}

async fn clients_state_from_global_clients() -> Result<ClientsState> {
    Ok(ClientsState {
        active_clients: get_client_labels().await,
    })
}

fn load_clients_state_from(path_buf: PathBuf) -> Result<ClientsState> {
    let file = File::open(path_buf)?;
    let reader = BufReader::new(file);
    let clients_state = serde_json::from_reader(reader)?;
    Ok(clients_state)
}

async fn load_clients_state() -> Result<ClientsState> {
    let user_dir = get_user_dir().await?;
    tracing::info!("add_client: user dir {}", user_dir);
    let path_buf = Path::new(&user_dir).join("clients_state.json");
    Ok(load_clients_state_from(path_buf)?)
}

fn save_clients_state_to(clients_state: ClientsState, path_buf: PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path_buf)?;
    serde_json::to_writer(&file, &clients_state)?;
    Ok(())
}

async fn save_clients_state(clients_state: ClientsState) -> Result<()> {
    let user_dir = get_user_dir().await?;
    tracing::info!("add_client: user dir {}", user_dir);
    let path_buf = Path::new(&user_dir).join("clients_state.json");
    save_clients_state_to(clients_state, path_buf)?;
    Ok(())
}

pub async fn get_client_count() -> usize {
    GLOBAL_CLIENTS.lock().await.len()
}

pub async fn get_client_labels() -> Vec<String> {
    GLOBAL_CLIENTS.lock().await.keys().cloned().collect()
}

pub async fn client_exists(label: &str) -> bool {
    GLOBAL_CLIENTS.lock().await.contains_key(label)
}

pub async fn get_client_by_label(label: &str) -> Result<Arc<Client>> {
    if GLOBAL_CLIENTS.lock().await.len() == 0 {
        return Err(anyhow!("join a federation first"));
    }

    let client = GLOBAL_CLIENTS
        .lock()
        .await
        .get(label)
        .ok_or(anyhow!("no such client"))?
        .lock()
        .await
        .clone();

    Ok(client)
}

pub async fn add_client(config_url: &str) -> Result<Arc<Client>> {
    // TODO: create label from sorted peer ids
    let federation_connect: WsFederationConnect = serde_json::from_str(config_url)?;
    let mut members = federation_connect.members;
    members.sort_by_key(|k| k.1.to_string());
    let to_serialise = WsFederationConnect { members: members };
    // sorting of members and reserialising will give us most likely the same label for
    // the same federation configs, but in slightly different format
    let sanitized_config = serde_json::to_string(&to_serialise)?;
    tracing::info!("Sanitized config: {}", sanitized_config);
    let label = &Hash::hash(sanitized_config.as_bytes()).to_hex()[0..12];
    if GLOBAL_CLIENTS.lock().await.contains_key(label) {
        return Err(anyhow!("Can't add client twice"));
    }
    let user_dir = get_user_dir().await?;
    tracing::info!("add_client: user dir {}", user_dir);
    let filename = Path::new(&user_dir).join(label);
    // TODO: use federation name as "tree"
    let db = SledDb::open(filename, label)?;
    // FIXME: just doing this twice so that I can report a better error
    if let Err(_) = serde_json::from_str::<WsFederationConnect>(&config_url) {
        return Err(anyhow!("Invalid federation QR / code"));
    }
    let client = Client::new(db.into(), &config_url, label).await?;
    client.client.fetch_all_coins().await;

    let client_arc = Arc::new(client);
    // for good measure, make sure the balance is updated (FIXME)
    GLOBAL_CLIENTS
        .lock()
        .await
        .insert(String::from(label), Mutex::new(client_arc.clone()));
    tracing::info!("Client added {}", label);
    save_clients_state(clients_state_from_global_clients().await?).await?;
    if GLOBAL_POLLER.lock().await.is_none() {
        tracing::info!("polling started");

        *GLOBAL_POLLER.lock().await = Some(tokio::spawn(async move {
            for (_key, value) in &*GLOBAL_CLIENTS.lock().await {
                tracing::info!("polling {}", value.lock().await.label);
                value.lock().await.poll().await;
            }
        }));
    }
    Ok(client_arc.clone())
}

pub async fn remove_client(label: &str) -> Result<()> {
    // Remove client at index

    GLOBAL_CLIENTS.lock().await.remove(label);
    tracing::info!("Client removed {}", label);

    if GLOBAL_CLIENTS.lock().await.len() == 0 {
        {
            // Kill poller, when there are no more clients anymore
            let poller = GLOBAL_POLLER.lock().await;
            tracing::info!("poller {:?}", poller);
            if let Some(handle) = poller.as_ref() {
                handle.abort();
            }

            tracing::info!("polling stopped");
        }
        *GLOBAL_POLLER.lock().await = None;
    }

    Ok(())
}

pub async fn delete_database(label: &str) -> Result<()> {
    // Wipe database
    if let Some(user_dir) = GLOBAL_USER_DIR.lock().await.as_ref() {
        let db_dir = Path::new(&user_dir).join(label);
        std::fs::remove_dir_all(db_dir)?;
    }
    Ok(())
}

pub async fn get_user_dir() -> Result<String> {
    let user_dir = GLOBAL_USER_DIR
        .lock()
        .await
        .as_ref()
        .ok_or(anyhow!("not initialized"))?
        .clone();
    Ok(user_dir)
}

pub async fn set_user_dir(user_dir: String) {
    *GLOBAL_USER_DIR.lock().await = Some(user_dir.clone());
    tracing::info!("set user dir {}", &user_dir);
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{client::ConfigKey, init_tracing};

    use std::{fs, io::Read};

    type TestResult<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    async fn setup() {
        init_tracing();
        tracing::info!("setting up api tests");
        set_user_dir(String::from("/tmp/ld")).await;
        assert_eq!(get_user_dir().await.unwrap(), String::from("/tmp/ld"));
    }

    #[tokio::test]
    async fn test_clients_state_serde() -> TestResult {
        let expected_state_json = "{\"activeClients\":[\"foo\",\"bar\",\"fish\"]}";
        let clients_state = ClientsState {
            active_clients: vec!["foo".to_string(), "bar".to_string(), "fish".to_string()],
        };

        if let Err(err) = fs::remove_file("/tmp/ld/clients_state_tmp.json") {
            tracing::info!("Error: {}... ignoring. This is normal", err);
        } // ignore error

        // load unsucessful

        let new_clients_state =
            load_clients_state_from(PathBuf::from("/tmp/ls/clients_state_tmp.json"));
        assert!(new_clients_state.is_err());

        // save
        save_clients_state_to(
            clients_state,
            PathBuf::from("/tmp/ld/clients_state_tmp.json"),
        )?;

        let mut file = File::open("/tmp/ld/clients_state_tmp.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        assert_eq!(contents, expected_state_json);

        // load

        let new_clients_state =
            load_clients_state_from(PathBuf::from("/tmp/ld/clients_state_tmp.json"))?;

        assert_eq!(
            new_clients_state.active_clients,
            vec!["foo".to_string(), "bar".to_string(), "fish".to_string()]
        );

        fs::remove_file("/tmp/ld/clients_state_tmp.json")?;

        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> TestResult {
        setup().await;
        let r = init(String::from("/tmp/ld")).await;
        assert!(r.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_clients() -> TestResult {
        setup().await;
        let expected_label = String::from("7d2ce1a7dec8");
        if let Err(err) = fs::remove_file("/tmp/ld/clients_state.json") {
            tracing::info!("Error: {}... ignoring. This is normal", err);
        } // ignore error
        let r = add_client("{\"members\":[[0,\"wss://fm-signet.sirion.io:443\"]]}").await;
        assert!(r.is_ok());
        assert_eq!(expected_label, r.unwrap().label);
        let r = add_client("{\"members\":[[0,\"wss://fm-signet.sirion.io\"]]}").await;
        assert!(r.is_err());

        let client = get_client_by_label(expected_label.as_str()).await;
        assert!(client.is_ok());
        assert_eq!(client.unwrap().label.clone(), expected_label);

        remove_client(expected_label.as_str()).await?;
        //delete_database(expected_label.as_str()).await?;

        fs::remove_file("/tmp/ld/clients_state.json");
        fs::remove_dir_all("/tmp/ld/7d2ce1a7dec8");
        Ok(())
    }
}
