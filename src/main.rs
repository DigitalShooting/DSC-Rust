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

// extern crate simplesvg;

extern crate tera;

extern crate time;

extern crate clap;

extern crate dotenv;


mod config;
mod discipline;
mod session;
mod helper;
mod dsc_manager;
mod device_api;
mod web;
mod database;
mod print;

use std::thread;

use std::path::Path;
use clap::{Arg, App};

use config::Config;
use dsc_manager::DSCManager;
use web::{Config as SocketConfig, socket};





// Main Entry Point
// 1. Parse Config, crash if not valid
// 2. Start DSC
fn main() {
    let matches = App::new("DSC")
                        .version("1.0")
                        .author("Jannik Lorenz <mail@janniklorenz.de>")
                        .about("Digital Shooting Client")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Path to mail json config file, if not present ./config/config.json will be used")
                            .required(false)
                            .takes_value(true))
                        .arg(Arg::with_name("modes")
                        .short("m")
                            .long("modes")
                            .value_name("DIR")
                            .help("Path to modes dir, if not present ./config/modes/ will be used")
                            .required(false)
                            .takes_value(true))
                          .get_matches();

    let config_dir = matches.value_of("config").unwrap_or("./config/config.json");
    let modes_dir = matches.value_of("modes").unwrap_or("./config/modes/");

    match Config::new(Path::new(config_dir), Path::new(modes_dir)) {
        Ok(config) => start_dsc(config),
        Err(err) => println!("Error in config: {}", err),
    }
}

// Start DSCManager and init websocket
fn start_dsc(config: Config) {
    let main_config = config.clone();
    let websocket_config = config.websocket.clone();

    // Init manager
    let (manager, manager_thread) = DSCManager::new(config);

    // Start websocket server
    let socket_config = SocketConfig {
        address_port: websocket_config.url.to_string()
    };
    socket::start_websocket(socket_config, main_config, manager).unwrap();

    // Run until manager (and socket?! TODO) finishes
    manager_thread.join().unwrap();
}
