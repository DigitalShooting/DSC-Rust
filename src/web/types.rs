use std::sync::{Arc, Mutex, mpsc};
use std::time::SystemTime;

use session::Session;
use config::Config as DSCConfig;



/// Array for channel for communication between a broadcast thread and the client threads
pub type ClientSenders = Arc<Mutex<Vec<mpsc::Sender<String>>>>;



/// TODO remove and just use parameters?
pub struct Config {
    pub address_port: String,
}



/// Base type for client -> server websocket packages
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RequestType {
    /// Trigger new target in the current session
    NewTarget,

    /// Start new session with given discipline
    SetDisciplin {name: String},

    /// Start new part with given id
    SetPart {name: String, force_new_part: bool},

    /// Print Current Session
    Print,

    /// Shutdown client
    Shutdown,

    /// Disable automatic paper acks for the current session
    DisablePaperAck,

    /// Move the paper and check its movement
    CheckPaper,
}

/// Base type for server -> client websocket packages
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SendType {
    /// Current session
    Session {session: Session},

    /// TODO replace with custom config struct for websockets
    /// which contains possible disicplines, line info, etc.
    Config {config: DSCConfig},

    // Log message
    Log {log: Log}
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LogLevel {
    Debug, Testing, Normal
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub message: String,
    pub date: SystemTime,
    pub level: LogLevel,
}

impl Log {
    pub fn new(message: String) -> SendType {
        SendType::Log {
            log: Log {
                message,
                date: SystemTime::now(),
                level: LogLevel::Normal
            }
        }
    }
}
