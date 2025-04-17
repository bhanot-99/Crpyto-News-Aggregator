pub mod news;

use axum::{Router, routing::get};

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Crypto News Aggregator API" }))
        .route("/health", get(|| async { "Server is running" }))
        .merge(news::routes())
}
