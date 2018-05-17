use std::sync::{mpsc};
use std::error::Error as StdError;

use session::ShotRaw;



/// Communication Commands from the Manager to the DeviceAPI.
/// Used to inform about config changes and stopping.
pub enum DeviceCommand {
    /// Will stop the DeviceAPI.
    Stop,

    /// Informs about a change in the part, we use it to move the paper on Haering Devices.
    NewPart,

    /// On ESA devices this will move the paper an checks the movement
    CheckPaper,
}

/// Communication channel to Manager object, to inform about new shots and errors.
pub enum Action {
    /// Send new detected shot to the Manger
    NewShot(ShotRaw),

    /// Send an error event that occured in the DeviceAPI to the Manager
    // Error(String),

    Error(Error),
}



#[derive(Debug)]
pub enum Error {
    PaperStuck,
    InvalidSerialPort,
}



/// Abstract Device to start and stop the DeviceAPI
pub trait API {
    /// Start DeviceAPI loop, this call will spawn a new thread in the DeviceAPI and returns.
    /// tx:     channel to send new shots and errors to
    /// rx:     channel to recive command from the manager
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<DeviceCommand>);
}
