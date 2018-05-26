use serde_json;

use super::schema::session;



#[derive(Queryable, Debug)]
pub struct DBSession {
    pub id: i32,
    pub line_id: i32,
    pub data: Option<serde_json::Value>,
}



#[derive(Insertable, Debug)]
#[table_name = "session"]
pub struct NewDBSession<'a> {
    pub line_id: &'a i32,
    pub data: Option<&'a serde_json::Value>,
}
