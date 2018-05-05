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


use std::sync::{Arc, Mutex};
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

    // Init manager
    let (mut manager, manager_thread) = DSCManager::new_with_default();


    // Start websocket server
    let config = socket::Config {
        address_port: "0.0.0.0:3008".to_string()
    };
    socket::start_websocket(config, manager);

    // Run until manager (and socket?! TODO) finishes
    manager_thread.join().unwrap();
}
