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
use std::io::{self, prelude::*, BufReader, Write};
use std::path::PathBuf;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    store: HashMap<String, String>,
    log: File,
}

impl KvStore {
    /// Creates a `KvStore`.
    // pub fn new() -> Self {
    //    Self {
    //        store: HashMap::new()
    //
    //    }
    //}

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        let command = Command::remove(key.clone());
        self.store.remove(&key);

        //write_to_file(&self.path, &self.log, command);
        Ok(())
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::set(key.clone(), value.clone());
        self.store.insert(key, value);

        write_to_file(&self.log, command);

        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<String> {
        match self.store.get(&key).cloned() {
            Some(res) => Ok(res),
            None => Err(MyError::KeyNotFound),
        }
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

        let mut store: HashMap<String, String> = restore_history(&file)?;

        Ok(KvStore { store, log: file })
    }
}

fn restore_history(file: &File) -> Result<HashMap<String, String>> {
    let buf_reader = BufReader::new(file);
    let mut history: Vec<Command> = Vec::new();
    for line in buf_reader.lines() {
        let line = line.unwrap();
        println!("{}", line);

        let history_command: Command =
            serde_json::from_str(&line).expect("Failed to parse serialised data.");
        history.push(history_command);
    }
    println!("Size of history {:?}", history.len());

    let mut store: HashMap<String, String> = HashMap::new();
    for command in history.iter() {
        match command {
            Command::Set { key, value } => store.insert(key.to_string(), value.to_string()),
            Command::Remove { key } => store.remove(key),
        };
    }
    Ok(store)
}

fn write_to_file(mut file: &File, command: Command) {
    let serialized: String = serde_json::to_string(&command).unwrap();
    file.write_all(serialized.as_bytes());
    file.write_all(b"\r\n");
    //std::fs::write(path, serialized).expect("Failed to write tickets to disk.");
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

//Error Management
#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}

/*impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred.")
    }
}*/

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, MyError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
