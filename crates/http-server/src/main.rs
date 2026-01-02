use axum::{Router, response::Html, routing::get};
use tower_http::cors::CorsLayer;

use crate::{extractors::state::AppState, routers::users};

mod exception;
mod extractors;
mod handlers;
mod routers;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    shared::tracing::subscribe();
    shared::env::load();

    let state = AppState::new().await;

    let app = Router::new()
        .route("/", get(async || "🦀 Hello !"))
        .route(
            "/docs/openapi.yml",
            get(async || include_str!("../docs/openapi.yml")),
        )
        .route(
            "/docs",
            get(async || Html(include_str!("../docs/openapi.html"))),
        )
        .merge(users::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("🦀 Server is stating on 🏝️: {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
