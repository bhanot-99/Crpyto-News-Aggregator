use axum::{Router, routing::get};
use crate::handlers::news_handler::fetch_crypto_news;

pub fn routes() -> Router {
    Router::new()
        .route("/news", get(fetch_crypto_news))
}
