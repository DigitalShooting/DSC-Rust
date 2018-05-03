// JSON encoding/ decoding
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// websocket lib
extern crate websocket;

// to form u8 array to i32 etc.
extern crate byteorder;

// generates random numbers (for demo device)
extern crate rand;

// extern crate time;
// extern crate num_rational;


mod discipline;
mod session;
mod helper;
mod dsc_manager;
mod device_api;
mod web;

// use std::sync::{mpsc};
// use std::thread;
// use std::time::Duration;


/*
// use std::sync::{Arc, Mutex, mpsc};

// use std::sync::mpsc::channel;
// use std::sync::mpsc::{self, TryRecvError, Sender, Receiver};
// use std::io::{self, BufRead};
*/


// use session::shot::*;
// use discipline::*;


use std::time::Duration;
use std::thread;

fn main() {
    start_dsc();
}



fn start_dsc() {
    use std::sync::mpsc;
    use std::thread;

    use dsc_manager::*;
    use web::*;

    // Init manager and Update channel
    let (on_update_tx, on_update_rx) = mpsc::channel::<Update>();
    let mut manager = DSCManager::new_with_default(on_update_tx);

    // This channel can be used to update the state of the manager
    let set_event_tx = manager.set_event_tx.clone();

    // Init default discipline and send it to the manager
    // TODO get from config
    let discipline = helper::dsc_demo::lg_discipline();
    set_event_tx.send(Event::SetDisciplin(discipline));

    // Spawn a thread for the manager run loop and start it
    let manager_thread = thread::spawn(move || {
        manager.start();
    });

    // Start websocket server
    let config = socket::Config {
        address_port: "0.0.0.0:3009".to_string()
    };
    // socket::start_websocket(config, set_event_tx, on_update_rx);

    // Run until manager (and socket?! TODO) finishes
    manager_thread.join().unwrap();
}
