// use std::error::Error;
use std::sync::{mpsc};

use session::*;

// use dsc_manager::DSCManager;

pub enum Action {
    NewShot(Shot),
    Stop,
    Error(String),
}

pub trait API {
    // fn start(&mut self, manager: &'a Arc<Mutex<DSCManager>>);
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<Action>);
    fn stop(&self);
}
