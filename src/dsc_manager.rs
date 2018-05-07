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

/// Indicated the current state of the current shot provider
enum ShotProviderState {
    /// We have a running shot provider
    Running(mpsc::Sender<DeviceCommand>),

    /// We have no running shot provider
    NotRunning,
}


/// DSCManager contains the current session and sets up the shot provider for the current
/// discipline. Events for the websockets are send by a channel, which will be created from the
/// websocket site. The communication between a shot provider and the mananger also happens over
/// channels.
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
        let discipline = manager_mutex.lock().unwrap().session.discipline.clone();
        manager_mutex.lock().unwrap().start_shot_provider(discipline);
        return thread::spawn(move || {
            loop {
                let mut manager = manager_mutex.lock().unwrap();
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

                thread::sleep(Duration::from_millis(100));
            }
            println!("end");
        });
    }



    pub fn new_target(&mut self) {
        println!("new_target");
    }
    pub fn set_disciplin(&mut self, discipline: Discipline) {
        self.start_shot_provider(discipline.clone());
        self.session = Session::new(discipline);
    }
    pub fn set_user(&mut self, user: User) {
        self.session.user = user;
        self.update_sessions();
    }
    pub fn set_team(&mut self, team: Team) {
        self.session.team = team;
        self.update_sessions();
    }
    pub fn set_club(&mut self, club: Club) {
        self.session.club = club;
        self.update_sessions();
    }
    pub fn set_part(&mut self, part: PartType) {

    }
    pub fn set_active_session(&mut self, index: ActiveSession) {

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
