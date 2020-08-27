//#![deny(missing_docs)]

//! Simple in-memory key/value storee responds to command line arguments

use std::collections::HashMap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, Write};
use std::path::PathBuf;
mod errors;
pub use errors::{MyError, Result};
use serde_json::de::IoRead;
use serde_json::StreamDeserializer;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
///  use kvs::KvStore;
///  use std::env::current_dir;
///
///
/// let mut store = KvStore::open(current_dir().unwrap()).unwrap();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned()).unwrap().unwrap();
/// assert_eq!(val, "value".to_owned());
/// ```
pub struct KvStore {
    store: HashMap<String, String>,
    log: File,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        KvStore::open(cwd.as_path())
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::remove(key.clone());
        match self.store.remove(&key) {
            Some(_x) => {
                write_to_file(&self.log, command)?;
                return Ok(());
            }
            None => return Err(MyError::KeyNotFound),
        }
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::set(key.clone(), value.clone());
        self.store.insert(key, value);

        write_to_file(&self.log, command)?;

        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        //let command = Command::get(key.clone());
        //write_to_file(&self.log, command)?;
        Ok(self.store.get(&key).cloned())
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path = path.into();
        std::fs::create_dir_all(&path)?;

        path.push("log");
        path.set_extension("json");

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open(&path)?;

        let buf_reader = BufReader::new(&file);
        let stream = serde_json::Deserializer::from_reader(buf_reader).into_iter::<Command>();

        let store: HashMap<String, String> = restore_history(stream)?;

        Ok(KvStore { store, log: file })
    }
}

/// Private Function that read a log file and returns an in-memory KvStore
fn restore_history(
    mut file: StreamDeserializer<IoRead<BufReader<&File>>, Command>,
) -> Result<HashMap<String, String>> {
    let mut store: HashMap<String, String> = HashMap::new();
    while let Some(command) = file.next() {
        match command? {
            Command::Set { key, value } => store.insert(key.to_string(), value.to_string()),
            Command::Remove { key } => store.remove(key.as_str()),
            _ => None,
        };
    }
    //println!("Size of history {:?}", history.len());

    Ok(store)
}

// Private helper function to write a command to the log file.
fn write_to_file(mut file: &File, command: Command) -> Result<()> {
    let serialized: String = serde_json::to_string(&command).unwrap();
    file.write_all(serialized.as_bytes())?;
    file.write_all(b"\r\n")?;
    //std::fs::write(path, serialized).expect("Failed to write tickets to disk.");
    Ok(())
}

/// Command is an enum with each possible command of the database. Each enum
/// command will be serialized to a log file and used as the basis for populating/
/// updating an in-memory key/value store.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
    Get { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    // fn get(key: String) -> Command {
    //     Command::Get { key }
    // }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
