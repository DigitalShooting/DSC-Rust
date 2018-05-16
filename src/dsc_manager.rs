use std::sync::{mpsc};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use session::*;
use discipline::*;
use device_api;
use device_api::api::{API, Action, DeviceCommand};
use config::Config;

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
    pub config: Config,
}

impl DSCManager {
    pub fn new_with_default(config: Config) -> (DSCManagerMutex, DSCManagerThread) {
        let discipline = config.default_discipline.clone();
        let session = Session::new(config.line.clone(), discipline);

        let (get_from_device_tx, get_from_device_rx) = mpsc::channel::<Action>();
        let manager = DSCManager {
            session,
            on_update_tx: None,
            get_from_device_tx, get_from_device_rx,
            shot_provider_state: ShotProviderState::NotRunning,
            config,
        };

        let manager_mutex = Arc::new(Mutex::new(manager));
        let thread = DSCManager::start(manager_mutex.clone());
        return (manager_mutex, thread);
    }



    /// Start the manager thread and return its JoinHandle
    /// manager_mutex:  DSCMangerMutex, will be locked befor every access in the run loop
    pub fn start(manager: DSCManagerMutex) -> DSCManagerThread {
        // Start default discipline
        if let Ok(mut manager) = manager.lock() {
            let discipline = manager.session.discipline.clone();
            manager.start_shot_provider(discipline)
        }

        // Start and return main manager worker thread.
        // This will check the device channel for new shots, adds them to the session and send an
        // update to the observer (over the on_update_tx channel).
        return thread::spawn(move || {
            loop {
                manager.lock().unwrap().check_device_channel();
                thread::sleep(Duration::from_millis(100));
            }
        });
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

    // Check the get_from_device_rx channel for new messages from the device (e.g. shots).
    // If we have some, we process them (e.g. add the shot to the session) and send an update to
    // the observer.
    fn check_device_channel(&mut self) {
        if let Ok(message) = self.get_from_device_rx.try_recv() {
            match message {
                Action::NewShot(shot_raw) => match self.session.get_active_discipline_part() {
                    Some(discipline_part) => {
                        self.session.add_shot_raw(shot_raw);
                        self.update_sessions();
                    }
                    None => println!("can no add shot, active_discipline_part nil"),
                },
                Action::Error(err) => {
                    println!("Error from device_api {:?}", err);

                    // TODO send error stuct
                    // if let Some(ref on_update_tx) = self.on_update_tx {
                    //     match on_update_tx.send(Update::Error(format!("{}", err))) {
                    //         Ok(_) => {},
                    //         Err(err) => println!("{}", err),
                    //     }
                    // }

                },
            }
        }
    }



    /// Start given shot provider, if we still have a running one, we stop it.
    //
    // discipline:      new discipline
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





pub trait UpdateManager {

    // Set a new discipline
    // Stops/ Start a new shot provider and inits a new session with the given discipline.
    //
    // discipline:      discipline to use
    fn set_disciplin(&mut self, discipline: Discipline);

    // Set a new discipline by name
    // Will search the config for a discipline with given name, if we have one, we set it as the
    // current discipline by calling set_disciplin.
    //
    // discipline_id:   id of the discipline (the filename from the config, without suffix)
    fn set_disciplin_by_name(&mut self, discipline_id: &str);



    // Set new target
    // If allowed in the current discipline part, we add a new part to the session of the same
    // type as the current one.
    fn new_target(&mut self);



    // Update the user of the current session
    //
    // user:    new user
    // fn set_user(&mut self, user: User);

    // Update the team of the current session
    //
    // team:    new team
    // fn set_team(&mut self, team: Team);

    // Update the club of the current session
    //
    // club:    new club
    // fn set_club(&mut self, club: Club);



    // Change to a different part, which has to be in the current discipline parts.
    //
    // part:    PartType string of the part we want to change to
    fn set_part(&mut self, part: PartType);

    // Change the active part, index has to be in the range of the sessions parts
    //
    // index:   Index of the part to change to
    fn set_active_part(&mut self, index: ActivePart);
}

impl UpdateManager for DSCManager {

    fn set_disciplin(&mut self, discipline: Discipline) {
        println!("Set discipline {:?}", discipline.id);
        self.start_shot_provider(discipline.clone());
        self.session = Session::new(self.config.line.clone(), discipline);
        self.update_sessions();
    }

    fn set_disciplin_by_name(&mut self, discipline_id: &str) {
        match self.config.get_discipline(discipline_id) {
            Some(discipline) => self.set_disciplin(discipline.clone()),
            None => println!("Discipline to set not found: {}", discipline_id),
        }
    }



    fn new_target(&mut self) {
        println!("new_target");
        self.update_sessions();
    }



    // fn set_user(&mut self, user: User) {
    //     self.session.user = user;
    //     self.update_sessions();
    // }
    // fn set_team(&mut self, team: Team) {
    //     self.session.team = team;
    //     self.update_sessions();
    // }
    // fn set_club(&mut self, club: Club) {
    //     self.session.club = club;
    //     self.update_sessions();
    // }


    fn set_part(&mut self, part: PartType) {
        println!("set_part {:?}", part);
        self.update_sessions();
    }
    fn set_active_part(&mut self, index: ActivePart) {
        println!("set_active_part {:?}", index);
        self.update_sessions();
    }
}
