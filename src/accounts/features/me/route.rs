use axum::{
    http::StatusCode,
    response::{IntoResponse, Json as ResponseJson},
    Extension,
};

use crate::{accounts::domain::account::Claims, shared::exception_filter::CustomError};

use super::query::{Query, QueryHandler};

#[utoipa::path(
	get,
	path = "/api/v1/auth/me",
	responses(
		(status = 200, body = [Account]),
		(status = 400)
	),
	tag = "auth"
)]
pub async fn me(
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, (StatusCode, CustomError)> {
    let handler = QueryHandler::new();

    match claims.sub.parse::<i32>() {
        Ok(id) => match handler.query(Query { id }) {
            Ok(account) => Ok((StatusCode::OK, ResponseJson(account))),
            Err(e) => Err((e.status, e)),
        },
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            CustomError {
                message: "Invalid user id error.".to_string(),
                name: "InvalidUserIdError".to_string(),
                status: StatusCode::UNAUTHORIZED,
            },
        )),
    }
}
