use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use super::command::{self, CommandHandler};

pub async fn create_task(
    Json(payload): Json<command::Command>,
) -> Result<impl IntoResponse, (StatusCode, String)> {

    let handler = CommandHandler::new();

    match handler.command(payload) {
        Ok(account) => Ok((StatusCode::CREATED, ResponseJson(account))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
