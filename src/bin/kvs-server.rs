use env_logger::Env;
use kvs::{KvStore, KvsEngine};
use kvs::{Result, Server};
use log::info;
use std::env::current_dir;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::process::exit;
use structopt::StructOpt;
use structopt::clap::arg_enum;

//const DEFAULT_ENGINE: Engine = Engine::kvs;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Opt {
    #[structopt(
    long = "addr",
    help = "Sets the server address",
    value_name = ADDRESS_FORMAT,
    default_value = DEFAULT_LISTENING_ADDRESS,
    parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(long, help = "Sets the storage engine", value_name = "ENGINE-NAME",
    possible_values = &Engine::variants(), case_insensitive = true)]
    engine: Option<String>,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Engine {
        kvs,
        sled
    }
}


fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Starting up");
    //let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    //info!("Storage engine: {}", engine);
    info!("Listening on {}", opt.addr);
    let engine = KvStore::open(current_dir()?)?;
    let server: Server<KvStore> = Server::new(engine);
    server.open(opt.addr)?;
    Ok(())
    // write engine to engine file
    //fs::write(current_dir()?.join("engine"), format!("{}", engine))?;

    /* match engine {
    Engine::kvs => run_with_engine(KvStore::open(current_dir()?)?, opt.addr),
     Engine::sled => run_with_engine(
         SledKvsEngine::new(sled::Db::start_default(current_dir()?)?),
         opt.addr,
     ),*/
}
