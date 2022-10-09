//! Sled implementation of the `Database` trait. It should not be used anymore since it has known
//! issues and is unmaintained. Please use `rocksdb` instead.

use anyhow::Result;
use fedimint_api::db::batch::{BatchItem, DbBatch};
use fedimint_api::db::IDatabase;
use fedimint_api::db::PrefixIter;
pub use sled;
use sled::transaction::TransactionError;
use std::path::Path;
use std::str;
use tracing::error;

#[derive(Debug)]
pub struct SledDatabase(sled::Db);

// TODO: interface this with IDatabaseContainer for easy database replacement
impl SledDatabase {
    pub fn open(db_path: impl AsRef<Path>) -> Result<SledDatabase, sled::Error> {
        Ok(SledDatabase(sled::open(db_path)?))
    }

    pub fn inner(&self) -> &sled::Db {
        &self.0
    }

    pub fn open_tree(&self, name: &str) -> Result<SledTree> {
        Ok(SledTree(self.inner().open_tree(name)?))
    }

    pub fn drop_tree(&self, name: &str) -> Result<bool> {
        Ok(self.inner().drop_tree(name)?)
    }

    pub fn tree_names(&self) -> Result<Vec<String>> {
        let tmp = self.inner().tree_names();
        let names = tmp
            .iter()
            .filter_map(|name| match str::from_utf8(name.to_vec().as_ref()) {
                Ok(value) => Some(String::from(value)),
                Err(e) => {
                    println!("Sled name={name:?} caused error: {e:?}");
                    None
                }
            });
        Ok(names.collect())
    }
}

#[derive(Debug)]
pub struct SledTree(sled::Tree);

impl SledTree {
    pub fn open(db_path: impl AsRef<Path>, tree: &str) -> Result<SledTree, sled::Error> {
        let db = sled::open(db_path)?.open_tree(tree)?;
        Ok(SledTree(db))
    }

    pub fn inner(&self) -> &sled::Tree {
        &self.0
    }
}

impl From<sled::Tree> for SledTree {
    fn from(db: sled::Tree) -> Self {
        SledTree(db)
    }
}

impl From<SledTree> for sled::Tree {
    fn from(db: SledTree) -> Self {
        db.0
    }
}

// TODO: maybe make the concrete impl its own crate
impl IDatabase for SledTree {
    fn raw_insert_entry(&self, key: &[u8], value: Vec<u8>) -> Result<Option<Vec<u8>>> {
        let ret = self.inner().insert(key, value)?.map(|bytes| bytes.to_vec());
        self.inner().flush().expect("DB failure");
        Ok(ret)
    }

    fn raw_get_value(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self
            .inner()
            .get(key)
            .map_err(anyhow::Error::from)?
            .map(|bytes| bytes.to_vec()))
    }

    fn raw_remove_entry(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let ret = self
            .inner()
            .remove(key)
            .map_err(anyhow::Error::from)?
            .map(|bytes| bytes.to_vec());
        self.inner().flush().expect("DB failure");
        Ok(ret)
    }

    fn raw_find_by_prefix(&self, key_prefix: &[u8]) -> PrefixIter<'_> {
        Box::new(self.inner().scan_prefix(key_prefix).map(|res| {
            res.map(|(key_bytes, value_bytes)| (key_bytes.to_vec(), value_bytes.to_vec()))
                .map_err(anyhow::Error::from)
        }))
    }

    fn raw_apply_batch(&self, batch: DbBatch) -> Result<()> {
        let batch: Vec<_> = batch.into();

        let ret = self
            .inner()
            .transaction::<_, _, TransactionError>(|t| {
                for change in batch.iter() {
                    match change {
                        BatchItem::InsertNewElement(element) => {
                            if t.insert(element.key.to_bytes(), element.value.to_bytes())?
                                .is_some()
                            {
                                error!("Database replaced element! {:?}", element.key);
                            }
                        }
                        BatchItem::InsertElement(element) => {
                            t.insert(element.key.to_bytes(), element.value.to_bytes())?;
                        }
                        BatchItem::DeleteElement(key) => {
                            if t.remove(key.to_bytes())?.is_none() {
                                error!("Database deleted absent element! {:?}", key);
                            }
                        }
                        BatchItem::MaybeDeleteElement(key) => {
                            t.remove(key.to_bytes())?;
                        }
                    }
                }

                Ok(())
            })
            .map_err(anyhow::Error::from);
        self.inner().flush().expect("DB failure");
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::init_tracing;
    use std::str;

    use std::sync::Once;

    type TestResult<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    static DB_FOLDER: &str = "clients.db";
    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            init_tracing();
        });
    }

    #[tokio::test]
    async fn test_database() -> TestResult {
        setup();
        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let db = SledDatabase::open(Path::new(path).join(DB_FOLDER));
        assert!(db.is_ok());
        let db = SledDatabase::open(Path::new(path).join(DB_FOLDER));
        assert!(db.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_tree() -> TestResult {
        setup();
        let tmp_dir = tmp_env::create_temp_dir().expect("cannot create temp dir");
        assert!(std::fs::metadata(&*tmp_dir).is_ok());
        let path = tmp_dir.to_str().unwrap();
        let db = SledDatabase::open(Path::new(path).join(DB_FOLDER));
        assert!(db.is_ok());

        let tree1 = db.as_ref().unwrap().open_tree("tree1");
        assert!(tree1.is_ok());

        let tree_names = db.as_ref().unwrap().tree_names().unwrap();

        assert!(tree_names.contains(&String::from("tree1")));

        // drop
        let r = db.as_ref().unwrap().drop_tree("tree1");
        assert!(r.is_ok());
        assert!(r.unwrap()); // true

        let tree_names = db.as_ref().unwrap().tree_names().unwrap();
        assert!(!tree_names.contains(&String::from("tree1")));

        // drop again
        let r = db.as_ref().unwrap().drop_tree("tree1");
        assert!(r.is_ok());
        assert!(!r.unwrap()); // false

        let tree_names = db.as_ref().unwrap().tree_names().unwrap();
        assert!(!tree_names.contains(&String::from("tree1")));

        Ok(())
    }
}
