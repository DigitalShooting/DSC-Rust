use std::error;
use std::fmt;
use std::io::Error as IOError;
use serde_json::Error as JSONError;
use std::path::PathBuf;

use discipline::DisciplineError;



#[derive(Debug)]
pub enum Error {
    DisciplineTargetNotFound(DisciplineError),
    DefaultDisciplineNotFound,
    FileNotFound(IOError),
    JSONParsing(JSONError),
    DisciplineParsing(PathBuf, Box<Error>),
    TargetParsing(PathBuf, Box<Error>),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::DisciplineTargetNotFound(_err) =>
                "Target not found",
            Error::DefaultDisciplineNotFound =>
                "Default discipline was not found",
            Error::FileNotFound(_err) =>
                "File not found",
            Error::JSONParsing(_err) =>
                "Error parsing JSON file",
            Error::DisciplineParsing(_, _) =>
                "Error parsing discipline json file",
            Error::TargetParsing(_, _) =>
                "Error parsing target json file",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::DisciplineTargetNotFound(ref e) => Some(e),
            Error::DefaultDisciplineNotFound => None,
            Error::FileNotFound(ref e) => Some(e),
            Error::JSONParsing(ref e) => Some(e),
            Error::DisciplineParsing(_, ref e) => Some(e),
            Error::TargetParsing(_, ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DisciplineTargetNotFound(_err) =>
                write!(f, "Target for Discipline not found"),
            Error::DefaultDisciplineNotFound =>
                write!(f, "Default discipline was not found"),
            Error::FileNotFound(err) =>
                write!(f, "File not found: {}", err),
            Error::JSONParsing(err) =>
                write!(f, "Error parsing JSON file: {}", err),
            Error::DisciplineParsing(path, err) =>
                write!(f, "Error parsing discipline json at path {:?}: {}", path, err),
            Error::TargetParsing(path, err) =>
                write!(f, "Error parsing target json at path {:?}: {}", path, err),
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
        Error::JSONParsing(err)
    }
}

impl From<DisciplineError> for Error {
    fn from(err: DisciplineError) -> Error {
        Error::DisciplineTargetNotFound(err)
    }
}
