//! Map sled crate
use crate::engine::KvsEngine;
use crate::{MyError, Result};
use std::path::PathBuf;

pub struct SledKvsEngine {
    store: sled::Db,
}

impl KvsEngine for SledKvsEngine {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value.as_bytes())?;
        self.store.flush()?;
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self
            .store
            .get(key)?
            .map(|v| v.to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }
    /// Remove a given key.
    fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(key)?.ok_or(MyError::KeyNotFound)?;
        self.store.flush()?;
        Ok(())
    }
}

impl SledKvsEngine {
    /// Creates a `SledKvsEngine` from `sled::Db`.
    pub fn new() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        SledKvsEngine::open(cwd.as_path())
    }

    /// Open the SledKvsEngine at a given path. Return the `SledKvsEngine`.
    pub fn open(path: impl Into<PathBuf>) -> Result<SledKvsEngine> {
        let mut path = path.into();
        std::fs::create_dir_all(&path)?;
        path.push("sled-db");
        Ok(SledKvsEngine {
            store: sled::open(path)?,
        })
    }
}
