use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::errors::{MyError, Result};
use log::info;
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

/// Key value store client
pub struct KvsClient {
    writer: BufWriter<TcpStream>,
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        info!("Try to connect");

        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        info!("Connected to {:?}", tcp_reader.peer_addr()?);

        Ok(KvsClient {
            writer: BufWriter::new(tcp_writer),
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
        })
    }

    /// Get the value of a given key from the server.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(msg) => Err(MyError::StringError(msg)),
        }
    }

    /// Set the value of a string key in the server.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;
        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_value) => Ok(()),
            SetResponse::Err(msg) => Err(MyError::StringError(msg)),
        }
    }

    /// Remove a string key in the server.
    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;
        let resp = RemoveResponse::deserialize(&mut self.reader)?;
        match resp {
            RemoveResponse::Ok(_value) => Ok(()),
            RemoveResponse::Err(msg) => Err(MyError::StringError(msg)),
        }
    }
}
