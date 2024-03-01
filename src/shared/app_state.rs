use std::sync::Mutex;

use diesel::PgConnection;

use super::db;

pub struct AppState {
    pub conn: Mutex<PgConnection>,
}

impl AppState {
    pub fn new() -> AppState {
        let conn = db::establish_connection();
        AppState { conn: conn.into() }
    }
}
