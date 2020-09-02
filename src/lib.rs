//#![deny(missing_docs)]

//! Simple in-memory key/value storee responds to command line arguments

use std::collections::{BTreeMap};
extern crate failure;
#[macro_use]
extern crate failure_derive;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::fs::{ File};
use std::io::{prelude::*, BufReader, BufWriter, SeekFrom, Write};
use std::ops::Range;
use std::path::PathBuf;
mod errors;
pub use errors::{MyError, Result};


/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::{MyError, Result};
/// # use kvs::KvStore;
/// # use std::env::current_dir;
/// # fn try_main() -> Result<()> {
///
/// let mut store = KvStore::new()?;
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
///
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
    writer: BufWriter<File>,
    reader: BufReader<File>,
    index: BTreeMap<String, Pointer>,
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
        self.writer.seek(SeekFrom::End(0))?;
        let command = Command::remove(key.clone());
        match self.index.remove(&key) {
            Some(_x) => {
                serde_json::to_writer(&mut self.writer, &command)?;
                self.writer.write_all(b"\r\n")?;
                self.writer.flush()?;
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
        let initial_offset = self.writer.seek(SeekFrom::End(0))?;
        self.writer.write_all(b"\r\n")?;
        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.flush()?;
        let new_offset = self.writer.seek(SeekFrom::End(0))?;
        self.index
            .insert(key.clone(), (initial_offset..new_offset).into());

        self.compact()?;
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.reader.seek(SeekFrom::Start(0))?;
        if let Some(pointer) = self.index.get(&key) {
            self.reader.seek(SeekFrom::Start(pointer.pos))?;
            let cmd_reader = (&mut self.reader).take(pointer.len);
            if let Command::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(MyError::KeyNotFound)
            }
        } else {
            Ok(None)
        }
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

        let mut kv = KvStore {
            writer: BufWriter::new(file),
            reader: BufReader::new(OpenOptions::new().read(true).open(&path)?),
            index: BTreeMap::new(),
            path,
        };

        kv.read_file()?;
        Ok(kv)
    }

    /// Read file and load history of command from the log
    pub fn read_file(&mut self) -> Result<()> {
        let mut buf_reader = BufReader::new(OpenOptions::new().read(true).open(&self.path)?);
        let mut initial_offset = buf_reader.seek(SeekFrom::Start(0))?;

        let mut stream = serde_json::Deserializer::from_reader(buf_reader).into_iter::<Command>();

        while let Some(command) = stream.next() {
            let new_offset = stream.byte_offset() as u64;
            match command? {
                Command::Set { key, .. } => {
                    self.index
                        .insert(key.to_string(), (initial_offset..new_offset).into());
                }
                Command::Remove { key } => {
                    self.index.remove(key.as_str());
                }
            };
            initial_offset = new_offset;
        }
        Ok(())
    }

    pub fn compact(&mut self)-> Result<()>{
        let mut path = std::env::current_dir()?;
        path.push("temp_log");
        path.set_extension("json");

        let temp_file= OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)?;

        let mut writer_temp_file = BufWriter::new(temp_file);
        self.reader.seek(SeekFrom::Start(0))?;
        for (_key,pointer) in &mut self.index {
            self.reader.seek(SeekFrom::Start(pointer.pos))?;
            let mut cmd_reader = (&mut self.reader).take(pointer.len);
            let _len = std::io::copy(&mut cmd_reader, &mut writer_temp_file)?;
        }
        writer_temp_file.flush()?;
        Ok(())
    }

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
    len: u64,
}

impl From<Range<u64>> for Pointer {
    fn from(range: Range<u64>) -> Self {
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
