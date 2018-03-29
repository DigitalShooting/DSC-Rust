// use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
// use std::error::Error;

use session::*;
// use dsc_manager::*;

use helper;

use api::API;
use api::Action;


pub struct Demo {
    // interval in which we generate shots (millisec.)
    interval: u64,
}

impl Demo {
    pub fn new() -> Demo {
        Demo { interval: 100_u64 }
    }

    fn generate_shot(tx: mpsc::Sender<Action>) {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::random(&target);
        tx.send(Action::NewShot(shot));
    }
}



impl API for Demo {
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<Action>) {

        let interval = self.interval;
        thread::spawn(move || {
            loop {
                match rx.try_recv() {
                    // Stop if we got a stop message or the channel disconnected
                    Ok(Action::Stop) | Err(TryRecvError::Disconnected) => {
                        println!("Stopping DeviceAPI");
                        break;
                    },
                    // When we got no message we generate a shot
                    Err(TryRecvError::Empty) => {
                        Demo::generate_shot(tx.clone());
                        thread::sleep(Duration::from_millis(interval));
                    }
                    _ => {},
                }
            }
        });
    }

    fn stop(&self) {

    }
}
