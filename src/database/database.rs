use std::env;
use diesel::{self, prelude::*, pg::PgConnection};
use dotenv::dotenv;
use serde_json;

use super::models::{DBSession, NewDBSession};

use session::Session as RSession;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn new_session_id(conn: &PgConnection, line_id: &i32) -> DBSession {
    use super::schema::session;

    let new_session = NewDBSession{line_id, data: None};
    diesel::insert_into(session::table)
        .values(&new_session)
        .get_result(conn)
        .expect("Error saving new session")
}

pub fn update_session(conn: &PgConnection, s: &RSession) {
    use super::schema::session::*;
    use super::schema::session::dsl::*;

    let dd = serde_json::to_value(s).unwrap();
    println!("{}", s.id);
    diesel::update(session.filter(id.eq(s.id)))
        .set(data.eq(Some(dd)))
        .execute(conn);
}











pub fn print_all_sessions() {
    use super::schema::session::dsl::*;

    let connection = establish_connection();
    let results = session.load::<DBSession>(&connection).expect("Error loading sessions");

    for session_object in results {
        println!("{}", session_object.id);
        println!("{}", session_object.line_id);
        println!("{:?}", session_object.data);
        println!("--------");
    }
}
