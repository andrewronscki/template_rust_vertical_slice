use axum::{
    routing::{get, post},
    Router,
};

use self::features::{create_task::route::create_task, find_task_by_id::route::find_task_by_id};

pub mod domain;
pub mod features;

pub fn tasks_routes() -> Router {
    Router::new()
        .route("/", post(create_task))
        .route("/:id", get(find_task_by_id))
}
