use axum::routing::{delete, get, post, put};
use axum::Router;
use std::net::SocketAddr;

async fn hello() -> &'static str {
    "Hello, World!"
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/hello", get(|| async { "Neuer Test" }))
}

pub async fn run_server(addr: SocketAddr) {
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
