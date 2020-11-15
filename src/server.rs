use crate::errors::{MyError, Result};
use log::{debug, error, info};
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use serde_json::Deserializer;

pub struct Server {}

impl Server {
    /* pub fn new() -> Server(){
        let listener = TcpListener::bind("127.0.0.1:80")?;
        Server{listener}
    }*/

    pub fn open() -> Result<()> {
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
pub fn handle_connections(mut stream: TcpStream) -> Result<()> {
    let peer_addr = stream.peer_addr()?;
    info!(
        "Connection established from {}, waiting for data...",
        stream.peer_addr()?
    );
    stream.local_addr();
    let reader = BufReader::new(&stream);
    let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

    for req in req_reader {
        info!("Receive request from {}: {:?}", peer_addr, req);
    }

    Ok(())
}