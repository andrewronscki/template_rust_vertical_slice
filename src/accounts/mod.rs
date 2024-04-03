use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::shared::jwt_auth::verify_jwt;

use self::features::{me::route::me, sign_in::route::sign_in, sign_up::route::sign_up};

pub mod domain;
pub mod features;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/sign-up", post(sign_up))
        .route("/sign-in", post(sign_in))
        .route("/me", get(me).layer(middleware::from_fn(verify_jwt)))
}
