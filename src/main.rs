#[macro_use]
extern crate diesel;

use crate::{accounts::auth_routes, tasks::tasks_routes};
use axum::routing::Router;
use docs::documentation::ApiDoc;
use shared::app_state::AppState;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dotenv::dotenv;
use std::env;

mod accounts;
mod docs;
mod shared;
mod tasks;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    AppState::new();

    match shared::messaging::establish_connection().await {
        Ok(_) => {
            log::info!("RabbitMQ Connected!")
        }
        Err(_) => {
            log::error!("Error to RabbitMQ connect!")
        }
    }

    let worker_handle = tokio::spawn(tasks::init_tasks_workers());

    let app = Router::new()
        .nest("/api/v1/tasks", tasks_routes())
        .nest("/api/v1/auth", auth_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CatchPanicLayer::new())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let port = env::var("APP_PORT").expect("APP_PORT must be set");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &port)
        .await
        .unwrap();

    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let _ = tokio::join!(worker_handle, server_handle);

    log::info!("Server started on port: {}", port);
}
