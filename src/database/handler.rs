use std::path::PathBuf;
use serde_json;
use std::fs::File;
use std::io::prelude::*;

// use std::error::Error;
use time::OffsetDateTime;

use session::Session;
use std::fs;


// Abstract Database interaction hander, used to generate new session ids and update session data.
// We don't use the observer solution (used in websocket server) here, because we also need a way
// to generate session ids.
pub trait DBHandler {
    // Generate new session id, with given line id.
    fn new_session_id(&self, line_id: i32) -> String;

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
    fn new_session_id(&self, _line_id: i32) -> String {
        return "0".to_string();
    }
    fn update_sesssion(&self, _session: &Session) {}
}







#[derive(Clone)]
pub struct DBHandlerFileSystem {
    path: PathBuf,
}
impl DBHandlerFileSystem {
    pub fn new(path: String) -> DBHandlerFileSystem {
        DBHandlerFileSystem{ path: PathBuf::from(path) }
    }
}
impl DBHandler for DBHandlerFileSystem {
    // TODO use i64, and use timestamp in ns
    fn new_session_id(&self, line_id: i32) -> String {
        let now = OffsetDateTime::now_local();
        
        return format!("{year}/{month}/{day}/Line_{line_id}_{houre}-{minute}-{second}_{millisecond}",
            line_id = line_id,
            year = now.year(),
            month = now.month(),
            day = now.day(),
            houre = now.hour(),
            minute = now.minute(),
            second = now.second(),
            millisecond = now.millisecond()
        ); 
    }
    fn update_sesssion(&self, session: &Session) {
        if session.id.contains(".") {
            panic!("forbidden char . in id");
        }
        
        let mut session_path = self.path.clone();
        session_path.push(format!("{sessionID}.{suffix}", sessionID = session.id.clone(), suffix = r"dscSession"));
        
        let session_path_parent = match session_path.parent() {
            None => panic!("could not create parent from path"),
            Some(path) => path,
        };
        match fs::create_dir_all(session_path_parent) {
            Err(why) => panic!("couldn't create: {:?}", why),
            Ok(file) => file,
        };
        
        let mut file = match File::create(&session_path) {
            Err(why) => panic!("couldn't create: {:?}", why),
            Ok(file) => file,
        };

        let text = serde_json::to_string(&session).unwrap();
        match file.write_all(text.as_bytes()) {
            Err(why) => panic!("couldn't write to: {:?}", why),
            Ok(_) => (),
        }
    }
}
