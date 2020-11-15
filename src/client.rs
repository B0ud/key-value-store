use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::errors::{MyError, Result};
use log::{debug, error, info};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::Write;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

/// Key value store client
pub struct KvsClient {}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        info!("Try to connect ");
        let mut tcp_reader = TcpStream::connect(addr)?;
        //let tcp_writer = tcp_reader.try_clone()?;
        let request : Request = Request::Get {  key: "titi".to_string() };
        let mut json = String::new();
        let test = serde_json::to_string_pretty(&request)?;

        tcp_reader.write(test.as_bytes());
        //let resp = GetResponse::deserialize(&mut self.reader)?;

        Ok(KvsClient {})
    }

    // /// Get the value of a given key from the server.
    /* pub fn get(&mut self, key: String) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;
        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => Ok(value),
            GetResponse::Err(msg) => Err(KvsError::StringError(msg)),
        }
    }*/
}
