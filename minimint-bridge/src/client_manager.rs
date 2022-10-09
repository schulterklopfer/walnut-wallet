use crate::client::Client;
use crate::database::SledDatabase;
use anyhow::{anyhow, Result};
use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::sha256::Hash;
use fedimint_api::BitcoinHash;
use mint_client::api::WsFederationConnect;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
extern crate lazy_static;

static CLIENT_DB_FILENAME: &str = "client.db";
static DEFAULT_TREE_ID: &str = "__sled__default";

pub struct ClientManager {
    clients: Arc<Mutex<HashMap<String, Mutex<Arc<Client>>>>>,
    poller: Mutex<Option<JoinHandle<()>>>,
    user_dir: Mutex<Option<String>>,
    db: Mutex<Option<SledDatabase>>,
}

// Justin: static function to return a reference to the federation you're working on.
// Dart side can call methods on it.

impl ClientManager {
    pub fn new() -> ClientManager {
        ClientManager {
            clients: Arc::new(Mutex::new(HashMap::new())),
            poller: Mutex::new(None),
            user_dir: Mutex::new(None),
            db: Mutex::new(None),
        }
    }

    pub async fn load(&self, path: &str) -> Result<()> {
        *self.user_dir.lock().await = Some(String::from(path));
        *self.db.lock().await = Some(SledDatabase::open(
            Path::new(path).join(CLIENT_DB_FILENAME),
        )?);
        self.load_clients().await?;
        Ok(())
    }

    async fn load_clients(&self) -> Result<()> {
        let client_labels = self.get_client_labels_from_db().await;
        let db_option = self.db.lock().await;
        if let Some(db) = db_option.as_ref() {
            for client_label in client_labels.iter() {
                //let client_db_filename = Path::new(&path).join(client_label);

                if let Some(client) =
                    Client::try_load(db.open_tree(client_label)?.into(), client_label.as_str())
                        .await?
                {
                    let client = Arc::new(client);

                    self.clients.lock().await.insert(
                        String::from(client_label.as_str()),
                        Mutex::new(client.clone()),
                    );
                    tracing::info!("loading client {}", client_label);
                } else {
                    tracing::info!("no database for client {}", client_label);
                }
            }

            if self.poller.lock().await.is_none() {
                tracing::info!("polling started");
                *self.poller.lock().await = Some(tokio::spawn({
                    let clients = self.clients.clone();
                    async move {
                        for (_key, value) in &*clients.lock().await {
                            tracing::info!("polling {}", value.lock().await.label);
                            value.lock().await.poll().await;
                        }
                    }
                }));
            }
        }
        Ok(())
    }

    pub async fn get_client_count(&self) -> usize {
        self.clients.lock().await.len()
    }

    pub async fn get_client_labels(&self) -> Vec<String> {
        self.clients.lock().await.keys().cloned().collect()
    }

    pub async fn get_client_labels_from_db(&self) -> Vec<String> {
        let db_option = self.db.lock().await;
        if let Some(db) = db_option.as_ref() {
            // every tree name except the default sled tree name is a client label
            match db.tree_names() {
                Ok(tree_names) => tree_names
                    .iter()
                    // not sure if this is nice... TODO: revisit
                    .filter(|name| *name != DEFAULT_TREE_ID)
                    .map(|name_ref| name_ref.clone())
                    .collect(),
                Err(_) => vec![],
            }
        } else {
            vec![]
        }
    }

    pub async fn client_exists(&self, label: &str) -> bool {
        self.clients.lock().await.contains_key(label)
    }

    pub async fn get_client_by_label(&self, label: &str) -> Result<Arc<Client>> {
        if self.clients.lock().await.len() == 0 {
            return Err(anyhow!("join a federation first"));
        }

        let client = self
            .clients
            .lock()
            .await
            .get(label)
            .ok_or(anyhow!("no such client"))?
            .lock()
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
        if self.clients.lock().await.contains_key(label) {
            return Err(anyhow!("Can't add client twice"));
        }
        // TODO: use federation name as "tree"
        if let Some(db_container) = &*self.db.lock().await {
            // FIXME: just doing this twice so that I can report a better error
            if let Err(_) = serde_json::from_str::<WsFederationConnect>(&config_url) {
                return Err(anyhow!("Invalid federation QR / code"));
            }
            let db = db_container.open_tree(label)?;
            let client = Client::new(db.into(), &config_url, label).await?;
            client.client.fetch_all_coins().await;

            let client_arc = Arc::new(client);
            // for good measure, make sure the balance is updated (FIXME)
            self.clients
                .lock()
                .await
                .insert(String::from(label), Mutex::new(client_arc.clone()));
            tracing::info!("Client added {}", label);
            if self.poller.lock().await.is_none() {
                tracing::info!("polling started");

                *self.poller.lock().await = Some(tokio::spawn({
                    let clients = self.clients.clone();
                    async move {
                        for (_key, value) in &*clients.lock().await {
                            tracing::info!("polling {}", value.lock().await.label);
                            value.lock().await.poll().await;
                        }
                    }
                }));
            }
            Ok(client_arc.clone())
        } else {
            Err(anyhow!("no database to use."))
        }
    }

    pub async fn remove_client(&self, label: &str) -> Result<()> {
        // Remove client at index

        self.clients.lock().await.remove(label);
        tracing::info!("Client removed {}", label);

        if self.clients.lock().await.len() == 0 {
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

        Ok(())
    }

    pub async fn delete_client_database(&self, label: &str) -> Result<bool> {
        // Wipe database for client
        // all tokens of client are
        if let Some(db) = &*self.db.lock().await {
            db.drop_tree(label)?;
        }
        Ok(false) // no tree was deleted, but no error? How?
    }

    pub async fn delete_database(&self) -> Result<()> {
        // Wipe database COMPLETELY!!
        // EVERYTHING IS GONE!!
        if let Some(user_dir) = self.user_dir.lock().await.as_ref() {
            std::fs::remove_dir_all(Path::new(&user_dir).join(CLIENT_DB_FILENAME))?;
        }
        Ok(())
    }

    pub async fn get_user_dir(&self) -> Result<String> {
        let user_dir = self
            .user_dir
            .lock()
            .await
            .as_ref()
            .ok_or(anyhow!("not initialized"))?
            .clone();
        Ok(user_dir)
    }
}

#[cfg(test)]
mod tests {

    use fedimint_api::db::{Database, IDatabase};
    use fs_extra::dir::{copy, CopyOptions};

    use super::*;
    use crate::client::ConfigKey;
    use crate::init_tracing;
    use crate::tests::TestResult;

    use std::sync::Once;

    static INIT: Once = Once::new();

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
        let client_manager = ClientManager::new();

        let r = client_manager.load(path).await;
        assert!(r.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_init_with_data() -> TestResult {
        setup();
        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let client_manager = ClientManager::new();
        let client_label1 = "7d2ce1a7dec8";
        let client_label2 = "0cde707b67f2";

        // create dummy entry in client tree so it can be restored in
        // client_manager.load(path)
        {
            if let Ok(db) = SledDatabase::open(Path::new(path).join(CLIENT_DB_FILENAME)) {
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
                        &String::from("{\"members\":[[0,\"ws://188.166.55.8:5000\"]]}"),
                    )?;
                }
            }
        }

        let r = client_manager.load(path).await;
        assert!(r.is_ok());

        let client_labels = client_manager.get_client_labels().await;
        assert!(client_labels.contains(&String::from(client_label1)));
        assert!(client_labels.contains(&String::from(client_label2)));

        Ok(())
    }

    #[tokio::test]
    async fn test_clients() -> TestResult {
        setup();

        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let client_manager = ClientManager::new();
        client_manager.load(path).await?;

        let expected_label = "7d2ce1a7dec8";

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

        client_manager.remove_client(expected_label).await?;

        Ok(())
    }
}
