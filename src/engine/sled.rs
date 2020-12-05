//! Map sled crate
use crate::engine::KvsEngine;
use crate::{MyError, Result};

pub struct SledKvsEngine{

}

impl KvsEngine for SledKvsEngine {

    fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!()
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        unimplemented!()
    }

    fn remove(&mut self, key: String) -> Result<()> {
        unimplemented!()
    }
}

impl SledKvsEngine {

}