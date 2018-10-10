use std::sync::{mpsc};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use session::{Session, Update as UpdateSession, PartType, ActivePart, AddShotRaw};
use discipline::*;
use device_api;
use device_api::api::{API, Action, DeviceCommand};
use config::{Config, DatabaseConfig};
use web::{SendType, Log};



pub type DSCManagerMutex = Arc<Mutex<DSCManager>>;
pub type DSCManagerThread = thread::JoinHandle<()>;



/// Indicated the current state of the current shot provider
enum ShotProviderState {
    /// We have a running shot provider
    Running(mpsc::Sender<DeviceCommand>),

    /// We have no running shot provider
    NotRunning,
}






trait DBHandler {
    fn new_session_id(&self, line_id: i32) -> i32;
    fn update_sesssion(&self, session: &Session);
}



#[derive(Clone)]
struct DBHandlerNone {}
impl DBHandlerNone {
    pub fn new() -> DBHandlerNone {
        DBHandlerNone{}
    }
}
impl DBHandler for DBHandlerNone {
    fn new_session_id(&self, line_id: i32) -> i32 {
        return 0_i32;
    }
    fn update_sesssion(&self, session: &Session) {}
}




use database::database;

#[derive(Clone)]
struct DBHandlerSQL {
    db_url: String,
}
impl DBHandlerSQL {
    pub fn new(db_config: &DatabaseConfig) -> DBHandlerSQL {
        DBHandlerSQL { db_url: db_config.db_url.clone() }
    }
}
impl DBHandler for DBHandlerSQL {
    fn new_session_id(&self, line_id: i32) -> i32 {
        let con = database::establish_connection();
        let s = database::new_session_id(&con, &line_id);
        // println!("{:?}", s);
        // database::print_all_sessions();
        s.id
    }

    fn update_sesssion(&self, session: &Session) {
        let con = database::establish_connection();
        let s = database::update_session(&con, session);
        // database::print_all_sessions();
    }
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
    db_handler: Box<DBHandler + Send>,

    // Channel for sending changes to the socket api
    pub on_update_tx: Option<mpsc::Sender<SendType>>,

    // The shot_provider writes to this channel, we read it here
    // used for new shot, error form device, etc
    get_from_device_tx: mpsc::Sender<Action>,
    get_from_device_rx: mpsc::Receiver<Action>,

    shot_provider_state: ShotProviderState,
    pub config: Config,
}

impl DSCManager {
    pub fn new(config: Config) -> (DSCManagerMutex, DSCManagerThread) {
        // Init db handler, based on config
        let db_handler: Box<DBHandler + Send> = match config.database {
            Some(ref db_config) => Box::new(DBHandlerSQL::new(db_config)),
            None => Box::new(DBHandlerNone::new()),
        };

        // TODO REMOVE and just init session in one method
        let discipline = config.default_discipline.clone();
        // let session_id = db_handler.new_session_id();
        let session = Session::new(0, config.line.clone(), discipline);

        let (get_from_device_tx, get_from_device_rx) = mpsc::channel::<Action>();
        let manager = DSCManager {
            session,
            db_handler,
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
    ///
    /// manager_mutex:  DSCMangerMutex, will be locked befor every access in the run loop
    pub fn start(manager: DSCManagerMutex) -> DSCManagerThread {
        // Start default discipline
        if let Ok(mut manager) = manager.lock() {
            let discipline = manager.session.discipline.clone();
            manager.set_disciplin(discipline)
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



    /// Send current session to the client
    fn update_sessions(&mut self) {
        // TODO ref
        self.db_handler.update_sesssion(&self.session);

        let session = self.session.clone();
        self.send_message_to_observer(SendType::Session { session })
    }

    /// Send given message to on_update channel (e.g. to websocket)
    ///
    /// message:    Message to send
    fn send_message_to_observer(&mut self, message: SendType) {
        if let Some(ref on_update_tx) = self.on_update_tx {
            let _ = on_update_tx.send(message);
        }
    }

    /// Check the get_from_device_rx channel for new messages from the device (e.g. shots).
    /// If we have some, we process them (e.g. add the shot to the session) and send an update to
    /// the observer.
    fn check_device_channel(&mut self) {
        if let Ok(message) = self.get_from_device_rx.try_recv() {
            match message {
                Action::NewShot(shot_raw) => {
                    self.session.add_shot_raw(shot_raw);
                    self.update_sessions();
                },
                Action::Error(err) => {
                    println!("Error from device_api {:?}", err);
                    self.send_message_to_observer(
                        Log::new(format!("{}", err))
                    )
                },
            }
        }
    }



    /// Start given shot provider, if we still have a running one, we stop it.
    ///
    /// discipline:      new discipline
    fn start_shot_provider(&mut self, discipline: Discipline) {
        self.stop_shot_provider();

        println!("Starting Shot Provider");

        // With this channel we can set stuff to the shot_provider
        // used to stop the device or trigger manual update (paper move, etc.)
        let (set_to_device_tx, set_to_device_rx) = mpsc::channel::<DeviceCommand>();

        match discipline.interface {
            Interface::ESA { port, on_part_band, on_shot_band } => {
                let mut shot_provider = device_api::ESA::new(
                    port, on_part_band, on_shot_band,
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
                let _ = tx.send(DeviceCommand::Stop);
            },
            ShotProviderState::NotRunning => {},
        }
        self.shot_provider_state = ShotProviderState::NotRunning;
    }


    /// Move the paper and check its movement
    pub fn check_paper(&mut self) {
        match self.shot_provider_state {
            ShotProviderState::Running(ref mut tx) => {
                let _ = tx.send(DeviceCommand::CheckPaper);
            },
            _ => {},
        }
    }

    /// Disable automatic paper check for this session
    pub fn disable_paper_ack(&mut self) {
        match self.shot_provider_state {
            ShotProviderState::Running(ref mut tx) => {
                let _ = tx.send(DeviceCommand::DisablePaperAck);
            },
            _ => {},
        }
    }

}





pub trait UpdateManager {

    /// Set a new discipline
    /// Stops/ Start a new shot provider and inits a new session with the given discipline.
    ///
    /// discipline:      discipline to use
    fn set_disciplin(&mut self, discipline: Discipline);

    /// Set a new discipline by name
    /// Will search the config for a discipline with given name, if we have one, we set it as the
    /// current discipline by calling set_disciplin.
    ///
    /// discipline_id:   id of the discipline (the filename from the config, without suffix)
    fn set_disciplin_by_name(&mut self, discipline_id: &str);
}

impl UpdateManager for DSCManager {

    fn set_disciplin(&mut self, discipline: Discipline) {
        println!("Set discipline {:?}", discipline.id);
        self.start_shot_provider(discipline.clone());

        let session_id = self.db_handler.new_session_id(self.config.line.id);
        self.session = Session::new(session_id, self.config.line.clone(), discipline);
        // TODO init session in db, and set session id
        // or do it in session itself?

        self.update_sessions();
    }

    fn set_disciplin_by_name(&mut self, discipline_id: &str) {
        match self.config.get_discipline(discipline_id) {
            Some(discipline) => self.set_disciplin(discipline.clone()),
            None => println!("Discipline to set not found: {}", discipline_id),
        }
    }

}

impl UpdateSession for DSCManager {

    fn new_target(&mut self, force: bool) {
        println!("new_target");
        // self.session.new_target(force);
        // self.update_sessions();

        if let Some(d_part) = self.session.get_active_discipline_part() {
            match d_part.enable_reset_to_new_target {
                true => {
                    let part_type = self.session.get_active_part().part_type.clone();
                    self.set_part(part_type, force);
                }
                false => {
                    // TODO message
                    println!("cannot do new_target, not allowed");
                }
            }
        }
        else {
            // TODO message
            println!("ERROR, no active part");
        }



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

    fn set_part(&mut self, part_type: PartType, force: bool) {
        println!("set_part {:?}", part_type);
        self.session.set_part(part_type, force);
        self.update_sessions();

        match self.shot_provider_state {
            ShotProviderState::Running(ref mut tx) => {
                let _ = tx.send(DeviceCommand::NewPart);
            },
            _ => {},
        }
    }
    fn set_active_part(&mut self, index: ActivePart, force: bool) {
        println!("set_active_part {:?}", index);
        self.session.set_active_part(index, force);
        self.update_sessions();
    }
}
