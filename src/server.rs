use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::engine::{KvStore, KvsEngine};
use crate::errors::{Result};

use log::{error, info};
use serde_json::Deserializer;
use std::env::current_dir;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct Server<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> Server<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self {
        Server { engine }
    }

    pub fn open<A: ToSocketAddrs>(self,  addr: A) -> Result<()> {
        // accept connections and process them serially
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_connections(stream)?;
                }
                Err(e) => error!("Connection failed {}", e),
            }
        }
        Ok(())
    }
}
pub fn handle_connections(stream: TcpStream) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    info!(
        "Connection established from {}, waiting for data..., {}",
        stream.peer_addr()?,
        stream.local_addr()?
    );

    let reader = BufReader::new(&stream);
    let mut bufwriter = BufWriter::new(&stream);
    let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

    let mut kvs = KvStore::open(current_dir()?)?;

    for req in req_reader {
        info!("Receive request from {}: {:?}", peer_addr, req);

        match req? {
            Request::Get { key } => {
                let response = match kvs.get(key) {
                    Ok(value) => GetResponse::Ok(value),
                    Err(err) => GetResponse::Err(err.to_string()),
                };
                serde_json::to_writer(&mut bufwriter, &response)?;
                bufwriter.flush()?;
                info!("Response sent to {:?}: {:?}", peer_addr, response);
            },
            Request::Set {key ,value} => {
                let response = match  kvs.set(key, value) {
                    Ok(_value) => SetResponse::Ok(()),
                    Err(err) => SetResponse::Err(err.to_string()),
                };
                serde_json::to_writer(&mut bufwriter, &response)?;
                bufwriter.flush()?;
                info!("Response sent to {:?}: {:?}", peer_addr, response);
            }
            Request::Remove {key} => {
                let response = match  kvs.remove(key) {
                    Ok(_value) => RemoveResponse::Ok(()),
                    Err(err) => RemoveResponse::Err(err.to_string()),
                };
                serde_json::to_writer(&mut bufwriter, &response)?;
                bufwriter.flush()?;
                info!("Response sent to {:?}: {:?}", peer_addr, response);
            }

        };
    }

    Ok(())
}
