use std::error::Error as StdError;
use std::fmt;



#[derive(Debug)]
pub enum Error {
    TargetNotFound,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TargetNotFound => "Target not found",
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TargetNotFound => write!(f, "Target not found"),
        }
    }
}
