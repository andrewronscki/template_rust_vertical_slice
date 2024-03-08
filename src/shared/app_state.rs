use super::db::{self, Pool};
use lapin::Channel;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

static INSTANCE: OnceCell<AppState> = OnceCell::new();

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool>,
    pub channel: Arc<Mutex<Option<Arc<Channel>>>>,
}

impl AppState {
    pub fn new() -> AppState {
        let pool = db::establish_connection();

        AppState {
            db_pool: Arc::new(pool),
            channel: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn set_channel(channel: Arc<Channel>) -> Result<(), Box<dyn std::error::Error>> {
        let app_state = AppState::get_instance();

        let mut channel_option = app_state.channel.lock().map_err(|e| e.to_string())?;

        *channel_option = Some(channel);

        Ok(())
    }

    pub fn get_instance() -> &'static AppState {
        INSTANCE.get_or_init(|| AppState::new())
    }
}
