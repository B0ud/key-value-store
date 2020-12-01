//#![deny(missing_docs)]



mod client;
mod common;
mod errors;
mod server;
mod engine;


extern crate failure;
#[macro_use]
extern crate failure_derive;

pub use client::KvsClient;
pub use errors::{MyError, Result};
pub use server::Server;
pub use engine::{KvStore, KvsEngine};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
