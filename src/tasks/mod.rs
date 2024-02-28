use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::app_state::AppState;

use self::features::create_task::route::create_task;

pub mod domain;
pub mod features;

pub fn tasks_routes(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_task))
        .layer(Extension(shared_state))
}
