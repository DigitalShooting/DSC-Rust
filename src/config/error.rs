use std::error;
use std::fmt;
use std::io::Error as IOError;
use serde_json::Error as JSONError;

use discipline::DisciplineError;



#[derive(Debug)]
pub enum Error {
    DisciplineTargetNotFound(DisciplineError),
    FileNotFound(IOError),
    JSONParsingError(JSONError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            // DSCError::DisciplineTargetNotFound { target_name } => "Target not found",
            Error::DisciplineTargetNotFound(err) => "Target not found",
            Error::FileNotFound(err) => "File not found",
            Error::JSONParsingError(err) => "Error parsing JSON file",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::DisciplineTargetNotFound(ref e) => Some(e),
            Error::FileNotFound(ref e) => Some(e),
            Error::JSONParsingError(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DisciplineTargetNotFound(err) => write!(f, "Target for Discipline not found: {}", err),
            Error::FileNotFound(err) => write!(f, "File not found: {}", err),
            Error::JSONParsingError(err) => write!(f, "Error parsing JSON file {}", err),
        }

    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::FileNotFound(err)
    }
}

impl From<JSONError> for Error {
    fn from(err: JSONError) -> Error {
        Error::JSONParsingError(err)
    }
}

impl From<DisciplineError> for Error {
    fn from(err: DisciplineError) -> Error {
        Error::DisciplineTargetNotFound(err)
    }
}
