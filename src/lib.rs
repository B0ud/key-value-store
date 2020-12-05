//#![deny(missing_docs)]

mod client;
mod common;
mod engine;
mod errors;
mod server;

extern crate failure;
#[macro_use]
extern crate failure_derive;

pub use client::KvsClient;
pub use engine::{KvStore, KvsEngine, SledKvsEngine};
pub use errors::{MyError, Result};
pub use server::Server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
