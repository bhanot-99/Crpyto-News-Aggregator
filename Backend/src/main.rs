mod routes;
mod utils;
mod handlers;
mod services;
mod models;

use axum::Router;
use hyper::Server;
use std::net::SocketAddr;
use utils::cors::get_cors_layer;

#[tokio::main]
async fn main() {
    let cors = get_cors_layer();

    let app = Router::new()
        .merge(routes::init_routes())
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
