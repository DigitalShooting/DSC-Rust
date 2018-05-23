use serde_json;
use serde_json::Error as JSONError;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::fmt;
use std::error;



#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Action {
    Ping,
    GetTicks { address: u8 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Answer {
    Pong,
    Ticks { address: u8, ticks: u16 },
    Error { error: String },
}



#[derive(Debug)]
pub enum Error {
    BandAckError(String),
    ConnectionError(IOError),
    JSONError(JSONError),
    DataError(Utf8Error),
    NoAnswer,
    InvalidAddress,
}
impl From<IOError> for Error { fn from(err: IOError) -> Error { Error::ConnectionError(err) }}
impl From<JSONError> for Error { fn from(err: JSONError) -> Error { Error::JSONError(err) }}
impl From<Utf8Error> for Error { fn from(err: Utf8Error) -> Error { Error::DataError(err) }}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BandAckError(_) =>
                "BandAckError",
            Error::ConnectionError(_) =>
                "ConnectionError",
            Error::JSONError(_) =>
                "JSONError",
            Error::DataError(_) =>
                "DataError",
            Error::NoAnswer =>
                "NoAnswer",
            Error::InvalidAddress =>
                "InvalidAddress",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ConnectionError(ref e) => Some(e),
            Error::JSONError(ref e) => Some(e),
            Error::DataError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BandAckError(ref error) =>
                write!(f, "BandAckError: {}", error),
            Error::ConnectionError(ref err) =>
                write!(f, "ConnectionError: {}", err),
            Error::JSONError(ref err) =>
                write!(f, "JSONError: {}", err),
            Error::DataError(ref err) =>
                write!(f, "DataError: {}", err),
            Error::NoAnswer =>
                write!(f, "NoAnswer"),
            Error::InvalidAddress =>
                write!(f, "InvalidAddress"),
        }

    }
}



pub fn get_ticks(band_ack_server: &str, device_address: u8) -> Result<u16, Error> {
    let mut stream = TcpStream::connect(band_ack_server)?;

    let action = Action::GetTicks { address: device_address };
    let action_json = serde_json::to_string(&action)?;
    let _ = stream.write(action_json.as_bytes());

    let mut buf = [0; 512];
    let read_bytes = stream.read(&mut buf)?;
    if read_bytes > 0 {
        let json_string = str::from_utf8(&buf[0..read_bytes])?;
        let answer: Answer = serde_json::from_str(&json_string)?;
        return match answer {
            Answer::Ticks { address, ticks } => {
                if address == device_address { Ok(ticks) }
                else { Err(Error::InvalidAddress) }
            },
            Answer::Error { error } => Err(Error::BandAckError(error)),
            Answer::Pong => Err(Error::BandAckError("Invalid Answer".to_string())),
        }
    }
    Err(Error::NoAnswer)
}



/// Ask band ack server for ticks, if we encounter a connection error, we try it some more times
///
/// band_ack_server     IP/ Port of the band ack server
/// device_address      Address of the band sensor device
pub fn ask_for_ticks(band_ack_server: &str, device_address: u8) -> Result<u16, Error> {
    let mut e = Error::NoAnswer;
    for _ in 0..3 {
        match get_ticks(band_ack_server, device_address) {
            Ok(ticks) => return Ok(ticks),
            Err(Error::ConnectionError(_)) => {},
            Err(err) => e = err,
        }
    }
    return Err(e)
}
