use std::sync::{mpsc};
use std::error::Error as StdError;
use std::fmt;

use session::ShotRaw;
use device_api::esa::esa::SerialError;
use device_api::esa::paper_ack::Error as PaperAckError;



/// Communication Commands from the Manager to the DeviceAPI.
/// Used to inform about config changes and stopping.
pub enum DeviceCommand {
    /// Will stop the DeviceAPI.
    Stop,

    /// Informs about a change in the part, we use it to move the paper on Haering Devices.
    NewPart,

    /// On ESA devices this will move the paper an checks the movement
    CheckPaper,

    /// Disable band ack checks after each paper movement
    DisablePaperAck,
}

/// Communication channel to Manager object, to inform about new shots and errors.
#[derive(Debug)]
pub enum Action {
    /// Send new detected shot to the Manger
    NewShot(ShotRaw),

    /// Send an error event that occured in the DeviceAPI to the Manager
    Error(Error),
}

impl StdError for Action {
    fn description(&self) -> &str {
        match *self {
            Action::NewShot(_) => "NewShot",
            Action::Error(_) => "Device Error"
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::NewShot(ref shot) => write!(f, "NewShot: {:?}", shot),
            Action::Error(ref err) => write!(f, "{}", err),
        }
    }
}



#[derive(Debug)]
pub enum Error {
    PaperStuck,
    PaperAck(PaperAckError),
    InvalidSerialPort(SerialError),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::PaperStuck => "PaperStuck",
            Error::PaperAck(_) => "PaperAck",
            Error::InvalidSerialPort(_) => "InvalidSerialPort"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::PaperStuck => write!(f, "PaperStuck"),
            Error::PaperAck(ref e) => write!(f, "PaperStuck: {}", e),
            Error::InvalidSerialPort(ref e) => write!(f, "InvalidSerialPort: {}", e),
        }
    }
}



/// Abstract Device to start and stop the DeviceAPI
pub trait API {
    /// Start DeviceAPI loop, this call will spawn a new thread in the DeviceAPI and returns.
    /// tx:     channel to send new shots and errors to
    /// rx:     channel to recive command from the manager
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<DeviceCommand>);
}
