use std::sync::{Arc, Mutex, mpsc};

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

    /// Shutdown client
    Shutdown,
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

    /// Error message for the client
    Error {error: String}
}
