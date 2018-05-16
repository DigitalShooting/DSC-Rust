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

use std::path::Path;



mod config;
mod discipline;
mod session;
mod helper;
mod dsc_manager;
mod device_api;
mod web;

use config::Config;
use dsc_manager::DSCManager;
use web::*;



fn main() {
    match Config::new(Path::new("./config/")) {
        Ok(config) => start_dsc(config),
        Err(err) => println!("Error in config: {}", err),
    }
}



fn start_dsc(config: Config) {
    // Init manager
    let (manager, manager_thread) = DSCManager::new_with_default(config);

    // Start websocket server
    let config = socket::Config {
        address_port: "0.0.0.0:3008".to_string()
    };
    socket::start_websocket(config, manager);

    // Run until manager (and socket?! TODO) finishes
    manager_thread.join().unwrap();
}
