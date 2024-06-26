use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use crate::shared::exception_filter::CustomError;

use super::command::{CommandHandler, CreateTaskCommand};

#[utoipa::path(
	post,
	path = "/api/v1/tasks",
	request_body = CreateTaskCommand,
	responses(
		(status = 201, body = [Task]),
		(status = 400)
	),
	tag = "tasks"
)]
pub async fn create_task(
    Json(payload): Json<CreateTaskCommand>,
) -> Result<impl IntoResponse, (StatusCode, CustomError)> {
    let handler = CommandHandler::new();

    match handler.command(payload) {
        Ok(account) => Ok((StatusCode::CREATED, ResponseJson(account))),
        Err(e) => Err((e.status, e)),
    }
}
