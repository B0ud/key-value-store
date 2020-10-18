use kvs::{KvStore, Result, MyError};
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;
use std::env::current_dir;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(StructOpt, Debug)]
#[structopt(
name = "kvs-client",
//global_settings = "&[\
//                          AppSettings::DisableHelpSubcommand,\
//                          AppSettings::VersionlessSubcommands]"
)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "get", about = "Get the string value of a given string key")]
    Get {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(
        long = "addr",
        help = "Sets the server address",
        value_name = "IP:PORT",
        default_value = DEFAULT_LISTENING_ADDRESS,
        parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    Set {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(name = "VALUE", help = "The string value of the key")]
        value: String,
        #[structopt(
        long = "addr",
        help = "Sets the server address",
        value_name = "IP:PORT",
        default_value = DEFAULT_LISTENING_ADDRESS,
        parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[structopt(name = "rm", about = "Remove a given string key")]
    Remove {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(
        long = "addr",
        help = "Sets the server address",
        value_name = "IP:PORT",
        default_value = DEFAULT_LISTENING_ADDRESS,
        parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let mut kvs = KvStore::open(current_dir()?)?;

    match opt.command {
        Command::Get { key, addr } => {
            //let mut client = KvsClient::connect(addr)?;
            if let Some(value) = kvs.get(key)? {
                println!("{}", value);
            } else {
                println!("{} ", MyError::KeyNotFound)
            }
        }
        Command::Set { key, value, addr } => {
            //let mut client = KvsClient::connect(addr)?;
            kvs.set(key, value)?;
        }
        Command::Remove { key, addr } => {
            // let mut client = KvsClient::connect(addr)?;
            kvs.remove(key)?;
        }
    }
    Ok(())
}