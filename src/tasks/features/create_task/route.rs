use std::sync::Arc;

use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};

use crate::{app_state::AppState, tasks::domain};

use super::command;

pub async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<command::Command>,
) -> Result<ResponseJson<domain::task::Task>, (StatusCode, String)> {
    let mut conn = state.conn.lock().unwrap();

    match command::create_task_command(&mut *conn, payload) {
        Ok(account) => Ok(Json(account)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
