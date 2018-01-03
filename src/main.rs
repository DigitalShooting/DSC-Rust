extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate websocket;

mod session;
mod discipline;
mod helper;
mod dsc_manager;
mod device_api;
mod web;


use std::sync::{Arc, Mutex};
use std::thread;
// use std::sync::mpsc::channel;
use std::time::Duration;

// use std::sync::mpsc::{self, TryRecvError, Sender, Receiver};
// use std::io::{self, BufRead};



// use session::shot::*;
use session::*;
// use discipline::*;
use dsc_manager::*;
use device_api::*;
use web::*;


fn main() {
    let manager = DSCManager::new_with_default();
    let manager_arc = Arc::new(Mutex::new(manager));


    let mut shot_provider = device_api::Demo::new(&manager_arc);
    shot_provider.start();

    socket::start_websocket(socket::Config { address_port: "127.0.0.1:3009".to_string()}, &manager_arc);


    loop {
        thread::sleep(Duration::from_secs(1));
        // println!("{:?}", manager_arc.lock().unwrap().session);
    }
}



/*
on_show_message
on_hide_message
on_shutdown
*/
