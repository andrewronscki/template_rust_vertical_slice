use std::sync::Arc;

use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use crate::shared::app_state::AppState;

use super::command::{self, CommandHandler};

pub async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<command::Command>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut conn = state.conn.lock().unwrap();

    let handler = CommandHandler::new();

    match handler.command(&mut *conn, payload) {
        Ok(account) => Ok((StatusCode::CREATED, ResponseJson(account))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
