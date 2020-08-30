//#![deny(missing_docs)]

//! Simple in-memory key/value storee responds to command line arguments

use std::collections::{BTreeMap, HashMap};
extern crate failure;
#[macro_use]
extern crate failure_derive;
use serde::{Deserialize, Serialize};
use serde_json;
use std::ops::Range;
use std::fs::OpenOptions;
use std::fs::{read, File};
use std::io::{prelude::*, BufReader, BufWriter, SeekFrom, Write};
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
    writer: BufWriter<File>,
    reader: BufReader<File>,
    index: HashMap<String, Pointer>,
    path: PathBuf,
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
                //self.write_to_file(command)?;
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
        self.store.insert(key.clone(), value.clone());
        let initial_offset = self.writer.seek(SeekFrom::End(0))?;
        serde_json::to_writer(&mut self.writer, &command)?;
        //self.writer.write_all(b"\r\n")?;
        self.writer.flush()?;
        let new_offset = self.writer.seek(SeekFrom::End(0))?;
        self.index.insert(key.clone(), (initial_offset .. new_offset ).into());
        //self.write_to_file(command)?;
        println!("{:?}", self.index);
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.reader.seek(SeekFrom::Start(0))?;
        println!("vector {:?}", self.index );
        if let Some(pointer) = self.index.get(&key) {
            self.reader.seek(SeekFrom::Start(pointer.pos))?;

            let mut de   = serde_json::Deserializer::from_reader(&mut self.reader);
            let cmd: Command = serde::de::Deserialize::deserialize(&mut de)?;

            if let Command::Set { value, .. } = cmd {
                Ok(Some(value))
            } else {
                Err(MyError::KeyNotFound)
            }
        } else {
            Ok(None)
        }

        //Ok(self.store.get(&key).cloned())
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path = path.into();
        std::fs::create_dir_all(&path)?;

        path.push("log");
        path.set_extension("json");

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(&path)?;

        let mut buf_reader = BufReader::new(OpenOptions::new().read(true).open(&path)?);
        let mut buf_writer = BufWriter::new(file);

        Ok(KvStore {
            store: HashMap::new(),
            writer: buf_writer,
            reader: buf_reader,
            index: HashMap::new(),
            path,
        })
    }

    pub fn read_file(&mut self) -> Result<()> {
        let mut buf_reader = BufReader::new(OpenOptions::new().read(true).open(&self.path)?);
        let mut initial_offset = buf_reader.seek(SeekFrom::Start(0))?;

        let mut stream =
            serde_json::Deserializer::from_reader(buf_reader).into_iter::<Command>();

        while let Some(command) = stream.next() {
            let new_offset = stream.byte_offset() as u64;
            match command? {
                Command::Set { key, value } => {
                    self.store.insert(key.to_string(), value.to_string());
                    self.index.insert(key.to_string(), (initial_offset .. new_offset).into());
                }
                Command::Remove { key } => {
                    self.store.remove(key.as_str());
                    self.index.remove(key.as_str());
                }
            };
            initial_offset = new_offset;
        }
        Ok(())
    }

}

/// Private Function that read a log file and returns an in-memory KvStore
fn restore_history(
    mut file: StreamDeserializer<IoRead<BufReader<File>>, Command>,
    buf_reader: BufReader<File>,
) -> Result<HashMap<String, String>> {
    let mut store: HashMap<String, String> = HashMap::new();
    while let Some(command) = file.next() {
        match command? {
            Command::Set { key, value } => store.insert(key.to_string(), value.to_string()),
            Command::Remove { key } => store.remove(key.as_str()),
        };
    }
    //println!("Size of history {:?}", history.len());

    Ok(store)
}

/// Command is an enum with each possible command of the database. Each enum
/// command will be serialized to a log file and used as the basis for populating/
/// updating an in-memory key/value store.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
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

/// Represents the position and length of a json-serialized command in the log.
#[derive(Clone, Debug)]
struct Pointer {
    pos: u64,
    len: u64
}

impl From<(Range<u64>)> for Pointer {
    fn from((range): (Range<u64>)) -> Self {
        Pointer {
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
