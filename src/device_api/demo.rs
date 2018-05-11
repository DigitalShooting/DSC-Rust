use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

use session::ShotRaw;
use device_api::api::{API, Action, DeviceCommand};



/// Demo DeviceAPI to debug and test DSC without a real DeviceAPI
pub struct Demo {
    /// interval in which we generate shots (millisec.)
    interval: u64,
    max_shots: Option<u32>,
}

impl Demo {
    pub fn new(interval: u64, max_shots: Option<u32>) -> Demo {
        Demo { interval, max_shots }
    }

    /// Generate a random shot and send an action to the manager.
    fn generate_shot(tx: mpsc::Sender<Action>) {
        let shot = ShotRaw::random();
        match tx.send(Action::NewShot(shot)) {
            Ok(_) => {},
            Err(err) => println!("{}", err),
        }
    }
}



impl API for Demo {
    fn start(&mut self, tx: mpsc::Sender<Action>, rx: mpsc::Receiver<DeviceCommand>) {

        let interval = self.interval;
        let max_shots = self.max_shots;
        thread::spawn(move || {
            let mut shots_generated = 0_u32;
            loop {
                // Check if we extended the shot limit
                if let Some(max_shots) = max_shots {
                    if max_shots <= shots_generated {
                        return;
                    }
                    shots_generated += 1;
                }

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
