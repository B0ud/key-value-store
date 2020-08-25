use std::io::{self};

//Error Management
#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "{}", _0)]
    DeserializeError(#[cause] serde_json::error::Error),
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> MyError {
        MyError::Io(err)
    }
}


impl From<serde_json::error::Error> for MyError {
    fn from(err: serde_json::error::Error) -> MyError {
        MyError::DeserializeError(err)
    }
}
/*impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred.")
    }
}*/

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, MyError>;
