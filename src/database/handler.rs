use session::Session;
use database::database;
use config::DatabaseConfig;

// Abstract Database interaction hander, used to generate new session ids and update session data.
// We don't use the observer solution (used in websocket server) here, because we also need a way
// to generate session ids.
pub trait DBHandler {
    // Generate new session id, with given line id.
    fn new_session_id(&self, line_id: i32) -> i32;

    // Update given session object in database
    fn update_sesssion(&self, session: &Session);
}

// Dummy database handler, used when no database real backend is used
#[derive(Clone)]
pub struct DBHandlerNone {}
impl DBHandlerNone {
    pub fn new() -> DBHandlerNone {
        DBHandlerNone{}
    }
}
impl DBHandler for DBHandlerNone {
    // TODO use i64, and use timestamp in ns
    fn new_session_id(&self, line_id: i32) -> i32 {
        return 0_i32;
    }
    fn update_sesssion(&self, session: &Session) {}
}

// Real database hander
#[derive(Clone)]
pub struct DBHandlerSQL {
    config: DatabaseConfig,
}
impl DBHandlerSQL {
    pub fn new(config: &DatabaseConfig) -> DBHandlerSQL {
        DBHandlerSQL { config: config.clone() }
    }
}
impl DBHandler for DBHandlerSQL {
    fn new_session_id(&self, line_id: i32) -> i32 {
        let con = database::establish_connection();
        let s = database::new_session_id(&con, &line_id);
        s.id
    }

    fn update_sesssion(&self, session: &Session) {
        let con = database::establish_connection();
        let s = database::update_session(&con, session);
        // database::print_all_sessions();
    }
}
