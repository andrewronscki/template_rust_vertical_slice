use std::sync::Arc;

use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    response::Json as ResponseJson,
};

use crate::{app_state::AppState, tasks::domain};

use super::query;

pub async fn find_task_by_id(
    Extension(state): Extension<Arc<AppState>>,
		Path(id): Path<i32>,
) -> Result<ResponseJson<domain::task::Task>, (StatusCode, String)> {
    let mut conn = state.conn.lock().unwrap();

    match query::find_task_query(&mut *conn, id) {
        Ok(account) => Ok(Json(account)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
