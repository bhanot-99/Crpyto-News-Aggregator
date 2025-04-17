use tower_http::cors::CorsLayer;
use axum::http::{Method, header::{CONTENT_TYPE, HeaderValue}};

pub fn get_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE])
}
