use serde_json;
use serde_json::Error as JSONError;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::fmt;
use std::error;
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use device_api::api::{Action as DeviceAction, Error as DeviceError};
use device_api::esa::esa::{ESA, SerialPort};



const MIN_PAPER_MOVE_DELTA: u16 = 200;
const PAPER_STUCK_MOVEMENT: u8 = 2;



#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Action {
    /// Ping Device Ack Server, will reply with a pong
    Ping,
    /// Ask for the current ticks of the device with given address
    GetTicks { address: u8 },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Answer {
    /// Reply to ping, simple connection test
    Pong,
    /// Reply for GetTicks, containt ticks and address
    Ticks { address: u8, ticks: u16 },
    /// Some error happend
    Error { error: String },
}



#[derive(Debug)]
pub enum Error {
    PaperAckError(String),
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
            Error::PaperAckError(_) => "PaperAckError",
            Error::ConnectionError(_) => "ConnectionError",
            Error::JSONError(_) => "JSONError",
            Error::DataError(_) => "DataError",
            Error::NoAnswer => "NoAnswer",
            Error::InvalidAddress => "InvalidAddress",
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
            Error::PaperAckError(ref error) =>
                write!(f, "PaperAckError: {}", error),
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




pub struct PaperMoveChecker {
    paper_ack_server: String,
    address: u8,
    ticks: u16,
}
impl PaperMoveChecker {

    pub fn new(paper_ack_server: String, address: u8) -> PaperMoveChecker {
        PaperMoveChecker{ paper_ack_server, address, ticks: 0 }
    }

    /// Calculate delta between 2 values, if the first value is larger, we use the difference to
    /// u16_max and add the second value. Otherwise just second - first.
    fn real_delta(a: u16, b: u16) -> u16 {
        if a > b {
            (<u16>::max_value()-a) + b
        }
        else {
            b - a
        }
    }



    /// Connect to the paper ack server and ask for the current ticks of the device with our address.
    ///
    /// paper_ack_server    IP/ Port of the paper ack server
    /// device_address      Address of the paper sensor device
    fn ask_for_ticks(&self) -> Result<u16, Error> {
        let mut stream = TcpStream::connect(&self.paper_ack_server)?;

        let action = Action::GetTicks { address: self.address };
        let action_json = serde_json::to_string(&action)?;
        let _ = stream.write(action_json.as_bytes());

        let mut buf = [0; 512];
        let read_bytes = stream.read(&mut buf)?;
        if read_bytes > 0 {
            let json_string = str::from_utf8(&buf[0..read_bytes])?;
            let answer: Answer = serde_json::from_str(&json_string)?;
            return match answer {
                Answer::Ticks { address, ticks } => {
                    if address == self.address { Ok(ticks) }
                    else { Err(Error::InvalidAddress) }
                },
                Answer::Error { error } => Err(Error::PaperAckError(error)),
                Answer::Pong => Err(Error::PaperAckError("Invalid Answer".to_string())),
            }
        }
        Err(Error::NoAnswer)
    }



    /// Ask paper ack server for ticks, if we encounter a connection error, we try it some more times
    ///
    /// paper_ack_server    IP/ Port of the paper ack server
    /// device_address      Address of the paper sensor device
    pub fn ask_for_ticks_retry(&self) -> Result<u16, Error> {
        let mut e = Error::NoAnswer;
        for _ in 0..3 {
            match self.ask_for_ticks() {
                Ok(ticks) => return Ok(ticks),
                Err(Error::ConnectionError(_)) => {},
                Err(err) => e = err,
            }
        }
        return Err(e)
    }



    // Calls the paper move server and asks if the paper has been moved recently
    //
    // return:  true if Ok, false, if no movement
    fn ask_for_paper_move(&mut self) -> Result<bool, Error> {
        let old_ticks = self.ticks;
        self.ticks = self.ask_for_ticks_retry()?;
        let delta = PaperMoveChecker::real_delta(old_ticks, self.ticks);
        println!("oldTicks: {}, newTicks: {}, delta: {}, has_movement: {}", old_ticks, self.ticks, delta, delta > MIN_PAPER_MOVE_DELTA);
        Ok(delta > MIN_PAPER_MOVE_DELTA)
    }



    // Open thread to check for paper movement
    // We try 3 times to move the paper, otherwise we send an error on the tx channel
    //
    // paper_move_checker
    // port:    Serial port, used to perform_band
    // tx:      Channel to send error message, if any
    // TODO IP/ Config for paper move server
    pub fn check(paper_move_checker: Arc<Mutex<PaperMoveChecker>>, port: SerialPort, tx: mpsc::Sender<DeviceAction>) {
        thread::spawn(move || {
            // Check 3 times if we have any movement
            for _ in 0..3 {
                // return and end this thrad if ok
                if let Ok(mut pmc) = paper_move_checker.lock() {
                    match pmc.ask_for_paper_move() {
                        Ok(true) => return,
                        Ok(false) => {},
                        Err(err) => tx.send(DeviceAction::Error(DeviceError::PaperAck(err))).unwrap(),
                    }
                }

                // try to move
                ESA::perform_band(port, PAPER_STUCK_MOVEMENT);
            }
            tx.send(DeviceAction::Error(DeviceError::PaperStuck)).unwrap();
        });
    }
}
