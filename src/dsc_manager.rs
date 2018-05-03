use std::sync::{mpsc};
use std::thread;
// use std::sync::mpsc::channel;
use std::time::Duration;
// use std::sync::mpsc::Sender;

use helper;

use session::*;
use discipline::*;
use device_api;
use device_api::api::{API, Action, DeviceCommand};

use serde_json;




// Used to send status updates from the manager to the client (socket api)
pub enum Update { // EventResponse?
    Data(String),
    Error(String),
}

// Used to send status updates from the manager to the client (socket api)
pub enum Event {
    NewTarget,
    SetDisciplin(Discipline),
    SetUser(User),
    SetTeam(Team),
    SetClub(Club),
    SetPart(PartType),
    SetSessionIndex(i32),
}

/// Indicated the current state of the current shot provider
enum ShotProviderState {
    /// We have a running shot provider
    Running(mpsc::Sender<DeviceCommand>),

    /// We have no running shot provider
    NotRunning,
}



pub struct DSCManager {
    pub session: Session,

    // Channel for sending changes to the socket api
    on_update_tx: mpsc::Sender<Update>,
    // pub on_update_rx: mpsc::Receiver<Update>,

    // This is a channel to send to the current shot provider
    pub set_event_tx: mpsc::Sender<Event>,
    set_event_rx: mpsc::Receiver<Event>,

    // The shot_provider writes to this channel, we read it here
    // used for new shot, error form device, etc
    get_from_device_tx: mpsc::Sender<Action>,
    get_from_device_rx: mpsc::Receiver<Action>,

    shot_provider_state: ShotProviderState,
}

impl DSCManager {
    pub fn new_with_default(on_update_tx: mpsc::Sender<Update>) -> DSCManager {
        let discipline = helper::dsc_demo::lg_discipline();
        let session = Session::new(discipline);

        let (get_from_device_tx, get_from_device_rx) = mpsc::channel::<Action>();
        let (set_event_tx, set_event_rx) = mpsc::channel::<Event>();
        return DSCManager {
            session,
            on_update_tx,
            set_event_tx, set_event_rx,
            get_from_device_tx, get_from_device_rx,
            shot_provider_state: ShotProviderState::NotRunning,
        }
    }

    // send current session data as json to the client (socket)
    fn update_sessions(&mut self) {
        let text = serde_json::to_string(&self.session).unwrap();
        self.on_update_tx.send(Update::Data(text)).unwrap()
    }

    // main run loop for manager
    pub fn start(&mut self) {
        loop {
            if let Ok(message) = self.get_from_device_rx.try_recv() {
                match message {
                    Action::NewShot(shot) => {
                        self.session.add_shot(shot);
                        self.update_sessions();
                    },
                    Action::Error(err) => {
                        println!("Error {:?}", err);
                    },
                    _ => {},
                }
            }

            if let Ok(message) = self.set_event_rx.try_recv() {
                match message {
                    Event::NewTarget => {

                    },
                    Event::SetDisciplin(discipline) => {
                        self.start_shot_provider(discipline.clone());
                        self.session = Session::new(discipline);
                    },
                    Event::SetUser(user) => {
                        println!("Set User {:?}", user);
                        self.session.user = user;
                        self.update_sessions();
                    },

                    Event::SetTeam(team) => {
                        self.session.team = team;
                        self.update_sessions();
                    },
                    Event::SetClub(club) => {
                        self.session.club = club;
                        self.update_sessions();
                    },
                    Event::SetPart(part_type) => {
                        println!("{}", part_type);
                        // TODO!!!
                    },
                    Event::SetSessionIndex(index) => {
                        println!("{}", index);
                        // TODO!!!
                    },
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    }


    /// Start given shot provider, if we still have a running one, we stop it.
    pub fn start_shot_provider(&mut self, discipline: Discipline) {
        self.stop_shot_provider();

        println!("Starting Shot Provider");

        // TODO get from device config
        // let mut shot_provider = device_api::Demo::new();
        let mut shot_provider = device_api::ESA::new("/dev/ttyS0".to_string(), discipline);
        // let mut shot_provider = device_api::ESA::new("/dev/pts/3".to_string());

        // With this channel we can set stuff to the shot_provider
        // used to stop the device or trigger manual update (paper move, etc.)
        let (set_to_device_tx, set_to_device_rx) = mpsc::channel::<DeviceCommand>();
        self.shot_provider_state = ShotProviderState::Running(set_to_device_tx);

        shot_provider.start(self.get_from_device_tx.clone(), set_to_device_rx);
    }

    /// Stop the current shot provider, if any
    fn stop_shot_provider(&mut self) {
        match self.shot_provider_state {
            ShotProviderState::Running(ref mut tx) => {
                println!("Stopping Shot Provider");
                tx.send(DeviceCommand::Stop);
            },
            ShotProviderState::NotRunning => {},
        }
        self.shot_provider_state = ShotProviderState::NotRunning;
    }


}
