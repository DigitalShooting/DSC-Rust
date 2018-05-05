use std::sync::{mpsc};
use std::thread;
use std::marker::Sync;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use helper;

use session::*;
use discipline::*;
use device_api;
use device_api::api::{API, Action, DeviceCommand};

use serde_json;


pub type DSCManagerMutex = Arc<Mutex<DSCManager>>;
pub type DSCManagerThread = thread::JoinHandle<()>;

/// Used to send status updates from the manager to the client (socket api)
pub enum Update { // EventResponse?
    Data(String),
    Error(String),
}

/// Used to send status updates from the manager to the client (socket api)
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
    pub on_update_tx: Option<mpsc::Sender<Update>>,

    // The shot_provider writes to this channel, we read it here
    // used for new shot, error form device, etc
    get_from_device_tx: mpsc::Sender<Action>,
    get_from_device_rx: mpsc::Receiver<Action>,

    shot_provider_state: ShotProviderState,
}

impl DSCManager {
    pub fn new_with_default() -> (DSCManagerMutex, DSCManagerThread) {
        let discipline = helper::dsc_demo::lg_discipline();
        let session = Session::new(discipline);

        let (get_from_device_tx, get_from_device_rx) = mpsc::channel::<Action>();
        let manager = DSCManager {
            session,
            on_update_tx: None,
            get_from_device_tx, get_from_device_rx,
            shot_provider_state: ShotProviderState::NotRunning,
        };

        let manager_mutex = Arc::new(Mutex::new(manager));
        let thread = DSCManager::start(manager_mutex.clone());
        return (manager_mutex, thread);
    }



    // send current session data as json to the client (socket)
    fn update_sessions(&mut self) {
        if let Some(ref on_update_tx) = self.on_update_tx {
            match serde_json::to_string(&self.session) {
                Ok(text) => {
                    match on_update_tx.send(Update::Data(text)) {
                        Ok(_) => {},
                        Err(err) => println!("{}", err),
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
    }

    /// Start the manager thread and return its JoinHandle
    /// manager_mutex:  DSCMangerMutex, will be locked befor every access in the run loop
    pub fn start(manager_mutex: DSCManagerMutex) -> DSCManagerThread {
        match manager_mutex.lock() {
            Ok(mut manager) => {
                let discipline = manager.session.discipline.clone();
                manager.start_shot_provider(discipline);
            },
            Err(err) => println!("Error {:?}", err),
        }
        return thread::spawn(move || {
            loop {
                match manager_mutex.lock() {
                    Ok(mut manager) => {
                        if let Ok(message) = manager.get_from_device_rx.try_recv() {
                            match message {
                                Action::NewShot(shot) => {
                                    manager.session.add_shot(shot);
                                    manager.update_sessions();
                                },
                                Action::Error(err) => {
                                    println!("Error {:?}", err);
                                },
                            }
                        }

                        // if let Ok(message) = manager.set_event_rx.try_recv() {
                        //     match message {
                        //         Event::NewTarget => {
                        //
                        //         },
                        //         Event::SetDisciplin(discipline) => {
                        //             manager.set_disciplin(discipline);
                        //         },
                        //         Event::SetUser(user) => {
                        //             println!("Set User {:?}", user);
                        //             manager.session.user = user;
                        //             manager.update_sessions();
                        //         },
                        //         Event::SetTeam(team) => {
                        //             manager.session.team = team;
                        //             manager.update_sessions();
                        //         },
                        //         Event::SetClub(club) => {
                        //             manager.session.club = club;
                        //             manager.update_sessions();
                        //         },
                        //         Event::SetPart(part_type) => {
                        //             println!("{}", part_type);
                        //             // TODO!!!
                        //         },
                        //         Event::SetSessionIndex(index) => {
                        //             println!("{}", index);
                        //             // TODO!!!
                        //         },
                        //     }
                        // }
                    }
                    Err(err) => println!("Error {:?}", err),
                }

                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    pub fn new_target(&mut self) {
        println!("new_target");
    }
    pub fn set_disciplin(&mut self, discipline: Discipline) {
        self.start_shot_provider(discipline.clone());
        self.session = Session::new(discipline);
    }


    /// Start given shot provider, if we still have a running one, we stop it.
    fn start_shot_provider(&mut self, discipline: Discipline) {
        self.stop_shot_provider();

        println!("Starting Shot Provider");

        // TODO get from device config
        let mut shot_provider = device_api::Demo::new();
        // let mut shot_provider = device_api::ESA::new("/dev/ttyS0".to_string(), discipline);
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
