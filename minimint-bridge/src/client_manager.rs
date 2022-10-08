use crate::client::Client;
use anyhow::{anyhow, Result};
use bitcoin::hashes::hex::ToHex;
use bitcoin::hashes::sha256::Hash;
use fedimint_api::BitcoinHash;
use fedimint_sled::SledDb;
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientManagerState {
    pub active_clients: Vec<String>,
}

pub struct ClientManager {
    clients: Arc<Mutex<HashMap<String, Mutex<Arc<Client>>>>>,
    poller: Mutex<Option<JoinHandle<()>>>,
    user_dir: Mutex<Option<String>>,
}

// Justin: static function to return a reference to the federation you're working on.
// Dart side can call methods on it.

impl ClientManager {
    pub fn new() -> ClientManager {
        ClientManager {
            clients: Arc::new(Mutex::new(HashMap::new())),
            poller: Mutex::new(None),
            user_dir: Mutex::new(None),
        }
    }

    pub async fn load(&self, path: &str) -> Result<()> {
        *self.user_dir.lock().await = Some(String::from(path));
        self.load_clients().await?;
        Ok(())
    }

    async fn load_clients(&self) -> Result<()> {
        let client_manager_state_result = self.load_client_manager_state().await;
        let path = self.get_user_dir().await?;
        if client_manager_state_result.is_ok() {
            for client_label in client_manager_state_result.unwrap().active_clients.iter() {
                let client_db_filename = Path::new(&path).join(client_label);
                let db = SledDb::open(client_db_filename, "client")?;

                if let Some(client) = Client::try_load(db.into(), client_label.as_str()).await? {
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

    async fn client_manager_state_from_clients(&self) -> Result<ClientManagerState> {
        Ok(ClientManagerState {
            active_clients: self.get_client_labels().await,
        })
    }

    fn load_client_manager_state_from(&self, path_buf: PathBuf) -> Result<ClientManagerState> {
        let file = File::open(path_buf)?;
        let reader = BufReader::new(file);
        let client_manager_state = serde_json::from_reader(reader)?;
        Ok(client_manager_state)
    }

    async fn load_client_manager_state(&self) -> Result<ClientManagerState> {
        let user_dir = self.get_user_dir().await?;
        tracing::info!("add_client: user dir {}", user_dir);
        let path_buf = Path::new(&user_dir).join("client_manager_state.json");
        Ok(self.load_client_manager_state_from(path_buf)?)
    }

    fn save_client_manager_state_to(
        &self,
        client_manager_state: ClientManagerState,
        path_buf: PathBuf,
    ) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path_buf)?;
        serde_json::to_writer(&file, &client_manager_state)?;
        Ok(())
    }

    async fn save_client_manager_state(
        &self,
        client_manager_state: ClientManagerState,
    ) -> Result<()> {
        let user_dir = self.get_user_dir().await?;
        tracing::info!("add_client: user dir {}", user_dir);
        let path_buf = Path::new(&user_dir).join("client_manager_state.json");
        self.save_client_manager_state_to(client_manager_state, path_buf)?;
        Ok(())
    }

    pub async fn get_client_count(&self) -> usize {
        self.clients.lock().await.len()
    }

    pub async fn get_client_labels(&self) -> Vec<String> {
        self.clients.lock().await.keys().cloned().collect()
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
        let user_dir = self.get_user_dir().await?;
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
        self.clients
            .lock()
            .await
            .insert(String::from(label), Mutex::new(client_arc.clone()));
        tracing::info!("Client added {}", label);
        self.save_client_manager_state(self.client_manager_state_from_clients().await?)
            .await?;
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

    pub async fn delete_database(&self, label: &str) -> Result<()> {
        // Wipe database
        if let Some(user_dir) = self.user_dir.lock().await.as_ref() {
            let db_dir = Path::new(&user_dir).join(label);
            std::fs::remove_dir_all(db_dir)?;
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

    use super::*;
    use crate::init_tracing;

    use std::io::Read;
    use std::sync::Once;

    type TestResult<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            init_tracing();
        });
    }

    #[tokio::test]
    async fn test_client_manager_state_serde() -> TestResult {
        // When tmp_dir is dropped this temporary dir will be removed
        setup();
        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();

        let client_manager_state_json_file = "client_manager_state_tmp.json";

        let expected_state_json = "{\"activeClients\":[\"foo\",\"bar\",\"fish\"]}";
        let client_manager_state = ClientManagerState {
            active_clients: vec!["foo".to_string(), "bar".to_string(), "fish".to_string()],
        };

        // load unsucessful
        let client_manager = ClientManager::new();
        let new_client_manager_state = client_manager
            .load_client_manager_state_from(Path::new(path).join(client_manager_state_json_file));
        assert!(new_client_manager_state.is_err());
        // save
        client_manager.save_client_manager_state_to(
            client_manager_state,
            Path::new(path).join(client_manager_state_json_file),
        )?;

        let mut file = File::open(Path::new(path).join(client_manager_state_json_file))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        assert_eq!(contents, expected_state_json);

        // load

        let new_client_manager_state = client_manager
            .load_client_manager_state_from(Path::new(path).join(client_manager_state_json_file))?;

        assert_eq!(
            new_client_manager_state.active_clients,
            vec!["foo".to_string(), "bar".to_string(), "fish".to_string()]
        );

        Ok(())
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

        client_manager.remove_client(expected_label).await?;

        Ok(())
    }
}
