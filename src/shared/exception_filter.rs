use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
    pub name: String,
    pub status: StatusCode,
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
                "name": self.name,
                "message": self.message,
        }));

        (self.status, body).into_response()
    }
}
