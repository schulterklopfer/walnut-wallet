use crate::client::Client;
use crate::database::SledDatabase;
use anyhow::{anyhow, Result};
use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::sha256::Hash;
use fedimint_api::BitcoinHash;
use mint_client::api::WsFederationConnect;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use tokio::task::JoinHandle;
extern crate lazy_static;

static DEFAULT_TREE_ID: &str = "__sled__default";

pub struct ClientManager {
    clients: Arc<RwLock<HashMap<String, RwLock<Arc<Client>>>>>, // Maybe no RwLock for client Arc at all?
    poller: Mutex<Option<JoinHandle<()>>>,
    db: Arc<SledDatabase>,
}

// Justin: static function to return a reference to the federation you're working on.
// Dart side can call methods on it.

impl ClientManager {
    pub fn new(db: Arc<SledDatabase>) -> ClientManager {
        ClientManager {
            clients: Arc::new(RwLock::new(HashMap::new())),
            poller: Mutex::new(None),
            db: db,
        }
    }

    pub async fn load(&self) -> Result<()> {
        self.load_clients().await?;
        Ok(())
    }

    async fn load_clients(&self) -> Result<()> {
        let client_labels = self.get_client_labels_from_db().await;
        for client_label in client_labels.iter() {
            //let client_db_filename = Path::new(&path).join(client_label);
            match Client::try_load(
                self.db.open_tree(client_label)?.into(),
                client_label.as_str(),
            )
            .await
            {
                Ok(client) => {
                    let client = Arc::new(client);
                    {
                        let mut writeLock =
                            self.clients.try_write().expect("client map lock poisoned");
                        writeLock.insert(
                            String::from(client_label.as_str()),
                            RwLock::new(client.clone()),
                        );
                        drop(writeLock);
                    }
                    tracing::info!("loading client {}", client_label);
                }
                Err(e) => return Err(e),
            }
        }

        /*
        if self.poller.lock().await.is_none() {
            tracing::info!("polling started");
            *self.poller.lock().await = Some(tokio::spawn({
                let clients = self.clients.clone();
                async move {
                    let readLock = clients.try_read().expect("client map lock poisoned");
                    for (_key, value) in &*readLock {
                        let clientReadLock = value.try_read().expect("client lock poisoned");
                        tracing::info!("polling {}", clientReadLock.label);
                        clientReadLock.poll().await;
                    }
                    drop(readLock);
                }
            }));
        }
         */

        Ok(())
    }

    pub async fn get_client_count(&self) -> usize {
        self.clients.read().await.len()
    }

    pub async fn get_client_labels(&self) -> Vec<String> {
        self.clients.read().await.keys().cloned().collect()
    }

    pub async fn get_client_labels_from_db(&self) -> Vec<String> {
        // every tree name except the default sled tree name is a client label
        match self.db.tree_names() {
            Ok(tree_names) => tree_names
                .iter()
                // not sure if this is nice... TODO: revisit
                .filter(|name| *name != DEFAULT_TREE_ID)
                .map(|name_ref| name_ref.clone())
                .collect(),
            Err(_) => vec![],
        }
    }

    pub async fn client_exists(&self, label: &str) -> bool {
        self.clients.read().await.contains_key(label)
    }

    pub async fn get_client_by_label(&self, label: &str) -> Result<Arc<Client>> {
        if self.clients.read().await.len() == 0 {
            return Err(anyhow!("join a federation first"));
        }

        let client = self
            .clients
            .read()
            .await
            .get(label)
            .ok_or(anyhow!("no such client"))?
            .read()
            .await
            .clone();

        Ok(client)
    }

    pub async fn add_client(&self, config_url: &str) -> Result<Arc<Client>> {
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
        if self.clients.read().await.contains_key(label) {
            return Err(anyhow!("Can't add client twice"));
        }
        // FIXME: just doing this twice so that I can report a better error
        if let Err(_) = serde_json::from_str::<WsFederationConnect>(&config_url) {
            return Err(anyhow!("Invalid federation QR / code"));
        }
        let db = self.db.open_tree(label)?;
        let client = Client::new(db.into(), &config_url, label).await?;
        let client_arc = Arc::new(client);
        if let Some(c) = client_arc.client.as_ref() {
            c.fetch_all_coins().await;
        }
        // for good measure, make sure the balance is updated (FIXME)
        let mut writeLock = self.clients.write().await;
        writeLock.insert(String::from(label), RwLock::new(client_arc.clone()));
        drop(writeLock);
        tracing::info!("Client added {}", label);
        /*
        if self.poller.lock().await.is_none() {
            tracing::info!("polling started");

            *self.poller.lock().await = Some(tokio::spawn({
                let clients = self.clients.clone();
                async move {
                    for (_key, value) in &*clients.read().await {
                        tracing::info!("polling {}", value.read().await.label);
                        value.read().await.poll().await;
                    }
                }
            }));
        }
         */
        return Ok(client_arc.clone());
    }

    pub async fn remove_client(&self, label: &str) -> Result<()> {
        // Remove client at index
        tracing::info!("waiting for write lock {}", label);
        let mut writeLock = self.clients.write().await;
        let value = writeLock.remove(label);
        drop(writeLock);
        if value.is_none() {
            return Err(anyhow!("client does not exist"));
        }
        tracing::info!("Client removed {}", label);

        /*
        if self.clients.read().await.len() == 0 {
            {
                // Kill poller, when there are no more clients anymore
                let poller = self.poller.lock().await;
                tracing::info!("poller {:?}", poller);
                if let Some(handle) = poller.as_ref() {
                    handle.abort();
                }

                tracing::info!("polling stopped");
            }
            *self.poller.lock().await = None;
        }
         */

        Ok(())
    }

    pub async fn save_user_data_for_client(&self, label: &str, user_data: &str) -> Result<()> {
        match self.get_client_by_label(label).await {
            Ok(client) => {
                client.save_user_data(user_data);
                return Ok(());
            }
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_user_data_for_client(&self, label: &str) -> Option<String> {
        match self.get_client_by_label(label).await {
            Ok(client) => client.fetch_user_data(),
            Err(_) => None,
        }
    }

    pub async fn delete_client_database(&self, label: &str) -> Result<bool> {
        // Wipe database for client
        // all tokens of client are
        self.db.drop_tree(label)?;
        Ok(false) // no tree was deleted, but no error? How?
    }
}

#[cfg(test)]
mod tests {

    use fedimint_api::db::Database;

    use super::*;
    use crate::client::{ConfigKey, ConnectionStatus};
    use crate::init_tracing;
    use crate::tests::TestResult;

    use std::path::Path;
    use std::sync::Once;

    static INIT: Once = Once::new();
    static CLIENT_DB_FILENAME: &str = "client.db";

    fn setup() {
        INIT.call_once(|| {
            init_tracing();
        });
    }

    #[tokio::test]
    async fn test_init() -> TestResult {
        setup();
        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let db = SledDatabase::open(Path::new(&path).join(CLIENT_DB_FILENAME))?;
        let client_manager = ClientManager::new(Arc::new(db));
        let r = client_manager.load().await;
        assert!(r.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_init_with_data() -> TestResult {
        setup();
        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let db = SledDatabase::open(Path::new(path).join(CLIENT_DB_FILENAME))?;

        let client_label1 = "7d2ce1a7dec8";
        let client_label2 = "0cde707b67f2";

        // create dummy entry in client tree so it can be restored in
        // client_manager.load(path)

        if let Ok(tree) = db.open_tree(client_label1) {
            let data: Database = tree.into();
            data.insert_entry(
                &ConfigKey,
                &String::from("{\"members\":[[0,\"wss://fm-signet.sirion.io:443\"]]}"),
            )?;
        }
        if let Ok(tree) = db.open_tree(client_label2) {
            let data: Database = tree.into();
            data.insert_entry(
                &ConfigKey,
                &String::from("{\"members\":[[0,\"ws://foobar.nonexisting\"]]}"),
            )?;
        }
        let client_manager = ClientManager::new(Arc::new(db));
        let r = client_manager.load().await;

        assert!(r.is_ok());

        let client_labels = client_manager.get_client_labels().await;
        assert!(client_labels.contains(&String::from(client_label1)));
        assert!(client_labels.contains(&String::from(client_label2)));

        if let Ok(client) = client_manager.get_client_by_label(client_label1).await {
            assert_eq!(client.connection_status, ConnectionStatus::Connected);
        } else {
            assert!(false);
        }

        if let Ok(client) = client_manager.get_client_by_label(client_label2).await {
            assert_eq!(client.connection_status, ConnectionStatus::NotConnected);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_clients() -> TestResult {
        setup();

        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let db = SledDatabase::open(Path::new(&path).join(CLIENT_DB_FILENAME))?;
        let client_manager = ClientManager::new(Arc::new(db));
        let r = client_manager.load().await;

        let expected_label = "7d2ce1a7dec8";
        let user_data: &str = "{\"foo\":\"bar\"}";

        let r = client_manager
            .add_client("{\"members\":[[0,\"wss://fm-signet.sirion.io:443\"]]}")
            .await;
        assert!(r.is_ok());
        assert_eq!(expected_label, r.unwrap().label);
        let r = client_manager
            .add_client("{\"members\":[[0,\"wss://fm-signet.sirion.io\"]]}")
            .await;
        assert!(r.is_err());

        let client = client_manager.get_client_by_label(expected_label).await;
        assert!(client.is_ok());
        assert_eq!(client.unwrap().label.clone(), expected_label);

        let client_labels = client_manager.get_client_labels().await;
        assert!(client_labels.contains(&String::from(expected_label)));

        let r = client_manager
            .save_user_data_for_client(expected_label, user_data)
            .await;

        assert!(r.is_ok());

        let v = client_manager
            .fetch_user_data_for_client(expected_label)
            .await;

        assert!(v.is_some());
        assert_eq!(v.unwrap(), user_data);

        client_manager.remove_client(expected_label).await?;

        Ok(())
    }
}
