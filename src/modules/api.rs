use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::Database;
use std::net::SocketAddr;
use std::sync::Arc;

async fn hello() -> &'static str {
    "Hello, World!"
}



pub async fn run_server(addr: SocketAddr) {
    
}
