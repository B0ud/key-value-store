use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::errors::{MyError, Result};
use log::{debug, error, info};
use serde_json::Deserializer;
use std::io::Read;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

pub struct Server {

}

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

   for req in req_reader {
        info!("Receive request from {}: {:?}", peer_addr, req);

       match req? {
            Request::Get {key} => {
                let response : GetResponse = GetResponse::Ok(Some(String::from("reponse55145")));
                info!("Test serveur");

                serde_json::to_writer(&mut bufwriter, &response)?;
                bufwriter.flush()?;
                info!("Response sent to {:?}: {:?}",peer_addr, response);
            }
           _ => info!("Default options")
       };
    }


    Ok(())
}
