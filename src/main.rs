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

use dsc_manager::*;
use session::Line;
use web::*;



fn main() {
    start_dsc();
}



fn start_dsc() {
    // Init manager
    let line = Line {
        id: "01".to_string(),
        name: "Linie 1".to_string(),
        short_name: "1".to_string(),
    };
    let (manager, manager_thread) = DSCManager::new_with_default(line);

    // Start websocket server
    let config = socket::Config {
        address_port: "0.0.0.0:3008".to_string()
    };
    socket::start_websocket(config, manager);

    // Run until manager (and socket?! TODO) finishes
    manager_thread.join().unwrap();
}
