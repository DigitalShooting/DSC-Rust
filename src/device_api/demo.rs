use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;



use session::*;
use dsc_manager::*;

use helper;

use device_api::api::API;



pub struct Demo<'a> {
    pub interval: i32,
    pub manager: &'a Arc<Mutex<DSCManager>>,
}

impl<'a> Demo<'a> {
    pub fn new(manager: &'a Arc<Mutex<DSCManager>>) -> Demo {
        Demo { interval: 100, manager }
    }

    fn generate_shot(manager: &'a Arc<Mutex<DSCManager>>) {
        let target = helper::dsc_demo::lg_target();
        let shot1 = Shot::from_cartesian_coordinates(-100, -100, &target);
        manager.lock().unwrap().new_shot(shot1);
    }
}



impl<'a> API for Demo<'a> {
    fn start(&mut self) {
        let manager = self.manager.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                // Demo::generate_shot(&manager);
            }
        });
    }

    fn stop(&self) {

    }
}
