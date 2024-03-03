use super::db::{self, Pool};
use once_cell::sync::OnceCell;
use std::sync::Arc;

static INSTANCE: OnceCell<AppState> = OnceCell::new();

pub struct AppState {
    pub db_pool: Arc<Pool>,
}

impl AppState {
    pub fn new() -> AppState {
        let pool = db::establish_connection(); // Garanta que isso retorne um Pool
        AppState {
            db_pool: Arc::new(pool),
        }
    }

    pub fn get_instance() -> &'static AppState {
        INSTANCE.get_or_init(|| AppState::new())
    }
}
