use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use crate::shared::exception_filter::CustomError;

use super::command::{CommandHandler, SignUpCommand};

#[utoipa::path(
	post,
	path = "/api/v1/auth/sign-up",
	request_body = SignUpCommand,
	responses(
		(status = 201, body = [Tokens]),
		(status = 400)
	),
	tag = "auth"
)]
pub async fn sign_up(
    Json(payload): Json<SignUpCommand>,
) -> Result<impl IntoResponse, (StatusCode, CustomError)> {
    let handler = CommandHandler::new();

    match handler.command(payload) {
        Ok(account) => Ok((StatusCode::CREATED, ResponseJson(account))),
        Err(e) => Err((e.status, e)),
    }
}
