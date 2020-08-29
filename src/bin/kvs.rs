use clap::Clap;
use kvs::{self, KvStore, MyError, Result};
use std::env::current_dir;

#[derive(Debug, Clap)]
pub struct SetParameter {
    /// A string key
    pub key: String,
    /// The string value of the key
    pub value: String,
}

#[derive(Clap, Debug)]
#[clap(name = "kvs")]
#[clap(author, about, version)]
pub enum Command {
    /// Sets a string key/value pair
    #[clap(name = "set")]
    Set(SetParameter),
    /// Get the string value of a given string key
    #[clap(name = "get")]
    Get {
        /// A string key
        key: String,
    },
    /// Remove a given key
    #[clap(name = "rm")]
    Rm {
        /// A string key
        key: String,
    },
}

/*#[derive(Debug, Clap)]
#[clap(name = "kvs")]
#[clap(version = "1.0", author = "Mehdi B. <mehdi.boudart@gmail.com>")]
pub struct ApplicationArguments {
    #[clap(subcommand)]
    pub command: Command,
}*/

fn main() -> Result<()> {
    let opt = Command::parse();
    //println!("{:?}", opt);
    let mut kvs = KvStore::open(current_dir()?)?;
    kvs.read_file()?;
    match opt {
        Command::Get { key } => {
            let result = kvs.get(key.clone())?;
            match result {
                Some(value) => println!("{}", value),
                None => println!("{} ", MyError::KeyNotFound),
            }
            std::process::exit(0);
        }
        Command::Rm { key } => {
            let result = kvs.remove(key.clone());
            match result {
                Ok(_x) => {
                    //println!("Record was removed, key : {}", key);
                    std::process::exit(0);
                }
                Err(e) => {
                    println!("{} ", e);
                    std::process::exit(1)
                }
            }
        }
        Command::Set(set_parameter) => {
            let result = kvs.set(set_parameter.key.clone(), set_parameter.value.clone());

            match result {
                Ok(_x) => {
                    //println!("Record was saved, key : {}, value : {}" , set_parameter.key, set_parameter.value);
                    std::process::exit(0);
                }
                Err(e) => {
                    println!("{} ", e);
                    std::process::exit(1)
                }
            }
        }
    }
    //kvs.save();
    //Ok(())
}
