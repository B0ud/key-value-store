use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::errors::{MyError, Result};
use crate::engine::{KvsEngine, KvStore};

use log::{debug, error, info};
use serde_json::Deserializer;
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::env::current_dir;

pub struct Server<E: KvsEngine> {
    engine: E,
}

impl <E: KvsEngine> Server<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self{
        Server { engine }
    }

    pub fn open(mut self) -> Result<()> {
        // accept connections and process them serially
        let listener = TcpListener::bind("127.0.0.1:4000")?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_connections(stream);
                }
                Err(e) => error!("Connection failed {}", e),
            }
        }
        Ok(())
    }
}
pub fn handle_connections( stream: TcpStream) -> Result<()> {
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
            Request::Get {key} => {
                let response = match kvs.get(key){
                    Ok(value) => GetResponse::Ok(value),
                    Err(err) => GetResponse::Err(err.to_string()),
                };
                serde_json::to_writer(&mut bufwriter, &response)?;
                bufwriter.flush()?;
                info!("Response sent to {:?}: {:?}",peer_addr, response);
            }
           _ => info!("Default options")
       };
    }


    Ok(())
}
