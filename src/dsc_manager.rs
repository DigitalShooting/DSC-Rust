// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::sync::mpsc::channel;
// use std::time::Duration;
use std::sync::mpsc::Sender;

use session::*;
use discipline::*;

use helper;
use serde_json;









pub struct DSCManager {
    session: Session,
    discipline: Discipline,
    pub on_change_channel: Option<Sender<String>>,
}

impl DSCManager {
    pub fn new_with_default() -> DSCManager {
        let session = Session::new();
        let discipline = helper::dsc_demo::lg_discipline();
        return DSCManager {
            session,
            discipline,
            on_change_channel: None,
        }
    }

    fn update_sessions(&mut self) {
        match self.on_change_channel {
            Some(ref dispatcher) => {
                let text = self.get_session_json();
                dispatcher.send(text).unwrap()
            },
            None => {},
        }
    }
}



pub trait ShotProviderAPI {
    fn new_shot(&mut self, Shot);
    fn new_target(&mut self);
    fn set_disciplin(&mut self, Discipline);
    fn set_user(&mut self, User);
    fn set_team(&mut self, Team);
    fn set_club(&mut self, Club);
    fn set_part(&mut self, PartType);
    fn set_session_index(&mut self, i32);
    // fn on_print(&self, PrintMode);
    // fn on_load_data(&self);

    fn get_session_json(&self) -> String;
}

impl ShotProviderAPI for DSCManager {

    fn new_shot(&mut self, shot: Shot) {
        println!("[ShotProviderAPI][on_shot]: {:?}", shot);
        self.session.add_shot(shot, &self.discipline);

        self.update_sessions();
        // match self.on_change_channel {
        //     Some(ref dispatcher) => {
        //         let text = self.get_session_json();
        //         dispatcher.send(text).unwrap()
        //     },
        //     None => {},
        // }

    }

    fn new_target(&mut self) {
        // TODO!!!
    }
    fn set_disciplin(&mut self, discipline: Discipline) {
        self.session = Session::new();
        self.discipline = discipline;
    }
    fn set_user(&mut self, user: User) {
        self.session.user = user;
    }
    fn set_team(&mut self, team: Team) {
        self.session.team = team;
    }
    fn set_club(&mut self, club: Club) {
        self.session.club = club;
    }
    fn set_part(&mut self, part_type: PartType) {
        println!("{}", part_type);
        // TODO!!!
    }
    fn set_session_index(&mut self, index: i32) {
        println!("{}", index);
        // TODO!!!
    }



    fn get_session_json(&self) -> String {
        return serde_json::to_string(&self.session).unwrap();
    }
}


// enum PrintMode {
//
// }
