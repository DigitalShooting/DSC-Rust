extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate time;
extern crate websocket;
extern crate serial;
extern crate rand;
extern crate num_rational;

mod session;
mod discipline;
mod helper;
mod dsc_manager;
mod device_api;
mod web;


// use std::sync::{Arc, Mutex, mpsc};
use std::sync::{mpsc};
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
    let (on_update_tx, on_update_rx) = mpsc::channel::<Update>();

    let mut manager = DSCManager::new_with_default(on_update_tx);
    let set_event_tx = manager.set_event_tx.clone();


    let discipline = helper::dsc_demo::lg_discipline();
    set_event_tx.send(Event::SetDisciplin(discipline));


    thread::spawn(move || {
        manager.start();
    });
    

    let config = socket::Config {
        address_port: "0.0.0.0:3009".to_string()
    };
    socket::start_websocket(config, set_event_tx, on_update_rx);

    loop {
        thread::sleep(Duration::from_secs(2));
    }
}









// loop {
//     if let Ok(message) = on_update_rx.try_recv() {
//         match message {
//             Update::Data(string) => {
//                 println!("New Data {:?}", string);
//             },
//             Update::Error(err) => {
//                 println!("Error {:?}", err);
//             },
//         }
//     }
//     // thread::sleep(Duration::from_millis(100));
//
//     thread::sleep(Duration::from_secs(2));
//
//     set_event_tx.send(Event::SetUser(User {
//         first_name: "Gast".to_string(),
//         last_name: "".to_string(),
//         id: "0".to_string(),
//     }));
// }


// let mut manager_arc = Arc::new(Mutex::new(manager));
// match manager_arc.lock() {
//     Ok(mut manager) => manager.start_shot_provider(),
//     Err(err) => println!("{:?}", err),
// }

/*
on_show_message
on_hide_message
on_shutdown
*/
