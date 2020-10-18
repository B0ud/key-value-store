use kvs::{Result};
use std::env::current_dir;
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;

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
    #[structopt(long, help = "Sets the storage engine", value_name = "ENGINE-NAME")]
    engine: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    println!("No Implemented");
    Ok(())
    //let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    //info!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    //info!("Storage engine: {}", engine);
    //info!("Listening on {}", opt.addr);

    // write engine to engine file
    //fs::write(current_dir()?.join("engine"), format!("{}", engine))?;

    /* match engine {
    Engine::kvs => run_with_engine(KvStore::open(current_dir()?)?, opt.addr),
     Engine::sled => run_with_engine(
         SledKvsEngine::new(sled::Db::start_default(current_dir()?)?),
         opt.addr,
     ),*/
}
