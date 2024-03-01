#[macro_use]
extern crate diesel;

use std::sync::Arc;

use crate::tasks::tasks_routes;
use axum::Router;
use shared::app_state::AppState;

mod shared;
mod tasks;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    let shared_state = Arc::new(app_state);

    let app = Router::new().nest("/api/v1/tasks", tasks_routes(shared_state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
