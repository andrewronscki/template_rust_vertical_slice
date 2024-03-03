use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
};

use crate::shared::exception_filter::CustomError;

use super::query::QueryHandler;

pub async fn find_task_by_id(
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, CustomError)> {
    let handler = QueryHandler::new();

    match handler.query(id) {
        Ok(account) => Ok((StatusCode::OK, ResponseJson(account))),
        Err(e) => Err((e.status, e)),
    }
}
