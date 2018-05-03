use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use session::Shot;
use helper;
use device_api::api::{API, Action, DeviceCommand};



/// Demo DeviceAPI to debug and test DSC without a real DeviceAPI
pub struct Demo {
    /// interval in which we generate shots (millisec.)
    interval: u64,
}

impl Demo {
    pub fn new() -> Demo {
        Demo { interval: 2000_u64 }
    }

    /// Generate a random shot and send an action to the manager.
    fn generate_shot(tx: mpsc::Sender<Action>) {
        let target = helper::dsc_demo::lg_target();
        let shot = Shot::random(&target);
        tx.send(Action::NewShot(shot));
    }
}



impl API for Demo {
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<DeviceCommand>) {

        let interval = self.interval;
        thread::spawn(move || {
            loop {
                match rx.try_recv() {
                    // Stop if we got a stop message or the channel disconnected
                    Ok(DeviceCommand::Stop) | Err(TryRecvError::Disconnected) => {
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
}
