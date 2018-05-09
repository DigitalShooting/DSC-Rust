use std::sync::{mpsc};
use std::thread;
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
/// TODO:
/// Should only manage new_discipline and init/ stop shot Provider
/// - Move all other action to session
pub struct DSCManager {
    pub session: Session,

    // Channel for sending changes to the socket api
    pub on_update_tx: Option<mpsc::Sender<Update>>,

    // The shot_provider writes to this channel, we read it here
    // used for new shot, error form device, etc
    get_from_device_tx: mpsc::Sender<Action>,
    get_from_device_rx: mpsc::Receiver<Action>,

    shot_provider_state: ShotProviderState,
    line: Line,
}

impl DSCManager {
    pub fn new_with_default(line: Line) -> (DSCManagerMutex, DSCManagerThread) {
        // TODO use default
        let discipline = helper::dsc_demo::lg_discipline();
        let session = Session::new(line.clone(), discipline);

        let (get_from_device_tx, get_from_device_rx) = mpsc::channel::<Action>();
        let manager = DSCManager {
            session,
            on_update_tx: None,
            get_from_device_tx, get_from_device_rx,
            shot_provider_state: ShotProviderState::NotRunning,
            line,
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
        manager_mutex.lock().unwrap().start_shot_provider(discipline.clone());
        return thread::spawn(move || {
            loop {
                let mut manager = manager_mutex.lock().unwrap();
                if let Ok(message) = manager.get_from_device_rx.try_recv() {
                    match message {
                        Action::NewShot(shot_raw) => match manager.session.get_active_discipline_part() {
                            Some(discipline_part) => {
                                // let target = discipline.target.clone();
                                // let shot = Shot::from_raw(shot_raw, &target, &discipline_part.count_mode);
                                // println!("{:?}", shot);
                                manager.session.add_shot_raw(shot_raw);
                                manager.update_sessions();
                            }
                            None => println!("can no add shot, active_discipline_part nil"),
                        },
                        Action::Error(err) => println!("Error {:?}", err),
                    }
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
        self.session = Session::new(self.line.clone(), discipline);
    }
    // pub fn set_user(&mut self, user: User) {
    //     self.session.user = user;
    //     self.update_sessions();
    // }
    // pub fn set_team(&mut self, team: Team) {
    //     self.session.team = team;
    //     self.update_sessions();
    // }
    // pub fn set_club(&mut self, club: Club) {
    //     self.session.club = club;
    //     self.update_sessions();
    // }
    // pub fn set_part(&mut self, part: PartType) {
    //
    // }
    // pub fn set_active_session(&mut self, index: ActiveSession) {
    //
    // }






    /// Start given shot provider, if we still have a running one, we stop it.
    fn start_shot_provider(&mut self, discipline: Discipline) {
        self.stop_shot_provider();

        println!("Starting Shot Provider");

        // With this channel we can set stuff to the shot_provider
        // used to stop the device or trigger manual update (paper move, etc.)
        let (set_to_device_tx, set_to_device_rx) = mpsc::channel::<DeviceCommand>();

        match discipline.clone().interface {
            Interface::ESA { port, on_part_band, on_shot_band } => {
                let mut shot_provider = device_api::ESA::new(
                    port, on_part_band, on_shot_band, discipline,
                );
                shot_provider.start(self.get_from_device_tx.clone(), set_to_device_rx);
            },

            Interface::Demo { interval, max_shots } => {
                let mut shot_provider = device_api::Demo::new(
                    interval, max_shots
                );
                shot_provider.start(self.get_from_device_tx.clone(), set_to_device_rx);
            },
        };

        self.shot_provider_state = ShotProviderState::Running(set_to_device_tx);
    }

    /// Stop the current shot provider, if any
    fn stop_shot_provider(&mut self) {
        match self.shot_provider_state {
            ShotProviderState::Running(ref mut tx) => {
                println!("Stopping Shot Provider");
                match tx.send(DeviceCommand::Stop) {
                    Ok(_) => {},
                    Err(err) => println!("{}", err),
                }
            },
            ShotProviderState::NotRunning => {},
        }
        self.shot_provider_state = ShotProviderState::NotRunning;
    }


}
