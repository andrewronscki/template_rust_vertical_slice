use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use crate::shared::exception_filter::CustomError;

use super::command::{CommandHandler, RefreshCommand};

#[utoipa::path(
	post,
	path = "/api/v1/auth/refresh",
	request_body = RefreshCommand,
	responses(
		(status = 201, body = [Tokens]),
		(status = 400)
	),
	tag = "auth"
)]
pub async fn refresh(
    Json(payload): Json<RefreshCommand>,
) -> Result<impl IntoResponse, (StatusCode, CustomError)> {
    let handler = CommandHandler::new();

    match handler.command(payload) {
        Ok(account) => Ok((StatusCode::CREATED, ResponseJson(account))),
        Err(e) => Err((e.status, e)),
    }
}
