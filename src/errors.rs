use std::io::{self};
use std::string;

//Error Management
#[derive(Fail, Debug)]
pub enum MyError {
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "{}", _0)]
    DeserializeError(#[cause] serde_json::error::Error),
    /// Error with a string message
    #[fail(display = "{}", _0)]
    StringError(String),
    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[fail(cause)] string::FromUtf8Error),
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
impl From<sled::Error> for MyError {
    fn from(err: sled::Error) -> MyError {
        MyError::Sled(err)
    }
}
impl From<string::FromUtf8Error> for MyError {
    fn from(err: string::FromUtf8Error) -> MyError {
        MyError::Utf8(err)
    }
}

/*impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred.")
    }
}*/

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, MyError>;
