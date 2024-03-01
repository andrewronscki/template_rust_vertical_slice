use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use super::query::QueryHandler;

pub async fn find_task_by_id(
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
		let handler = QueryHandler::new();

    match handler.query(id) {
        Ok(account) => Ok((StatusCode::OK, ResponseJson(account))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
