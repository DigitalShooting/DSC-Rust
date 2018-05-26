// JSON encoding/ decoding
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// websocket lib
extern crate websocket;

// to form u8 array to i32 etc.
extern crate byteorder;

// generates random numbers (for demo device)
extern crate rand;

// extern crate time;

extern crate simplesvg;

#[macro_use]
extern crate diesel;
extern crate dotenv;



mod config;
mod discipline;
mod session;
mod helper;
mod dsc_manager;
mod device_api;
mod web;
mod database;

use config::Config;
use dsc_manager::DSCManager;
use web::{Config as SocketConfig, socket};

use std::path::Path;

fn main() {

    // database::database::print_all_sessions();
    // database::database::test_create_session();
    // database::database::print_all_sessions();

    match Config::new(Path::new("./config/")) {
        Ok(config) => start_dsc(config),
        Err(err) => println!("Error in config: {}", err),
    }
}



fn start_dsc(config: Config) {
    // Init manager
    let (manager, manager_thread) = DSCManager::new(config);

    // Start websocket server
    let config = SocketConfig {
        address_port: "0.0.0.0:3008".to_string()
    };
    socket::start_websocket(config, manager).unwrap();

    // Run until manager (and socket?! TODO) finishes
    manager_thread.join().unwrap();
}
