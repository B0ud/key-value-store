use env_logger::{Env, Target};
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use kvs::{Result, Server};
use log::info;
use std::env::current_dir;
use std::net::SocketAddr;
use std::process::exit;
use structopt::clap::arg_enum;
use structopt::StructOpt;

//const DEFAULT_ENGINE: Engine = Engine::kvs;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";
const DEFAULT_ENGINE: Engine = Engine::kvs;

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
    engine: Option<Engine>,
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .target(Target::Stdout)
        .init();

    info!("Starting up");
    //let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    //info!("Storage engine: {}", engine);
    info!("Listening on {}", opt.addr);

    let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);

    match engine {
        Engine::kvs => run_engine(KvStore::open(current_dir()?)?, opt.addr),
        Engine::sled => run_engine(SledKvsEngine::open(current_dir()?)?, opt.addr),
    }
}

fn run_engine<E: KvsEngine>(engine: E, addr: SocketAddr) -> Result<()> {
    let server = Server::new(engine);
    server.open(addr)
}
