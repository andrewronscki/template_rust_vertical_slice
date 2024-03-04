#[macro_use]
extern crate diesel;

use crate::tasks::tasks_routes;
use axum::Router;
use docs::documentation::ApiDoc;
use shared::app_state::AppState;
use tower_http::catch_panic::CatchPanicLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod docs;
mod shared;
mod tasks;

#[tokio::main]
async fn main() {
    AppState::new();

    let app = Router::new()
        .nest("/api/v1/tasks", tasks_routes())
        .layer(CatchPanicLayer::new())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
