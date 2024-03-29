use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use crate::shared::exception_filter::CustomError;

use super::query::QueryHandler;

#[utoipa::path(
	get,
	path = "/api/v1/tasks/{id}",
	params(("id" = usize, Path, description = "ID da tarefa")),
	responses(
		(status = 201, body = [Task]),
		(status = 400)
	),
	tag = "tasks"
)]
pub async fn find_task_by_id(
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, CustomError)> {
    let handler = QueryHandler::new();

    match handler.query(id) {
        Ok(account) => Ok((StatusCode::OK, ResponseJson(account))),
        Err(e) => Err((e.status, e)),
    }
}
