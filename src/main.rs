use anyhow::Error;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use axum::{extract, Json};
use dotenv::dotenv;
use modules::contact;
use modules::database::Database;
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use std::{env, net::SocketAddr, time::Duration};

mod modules;

use crate::modules::contact::Contact;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("ERROR: Expected \"DB_URL\"");

    let database = Database::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await?,
    );

    let shared_state = Arc::new(database);

    shared_state.create_table().await;

    let app = axum::Router::new()
        .route("/", get(index))
        .route("/api/contacts", get(contacts))
        .route("/api/contacts/id/:id", get(contact_by_id))
        .route("/api/contacts/name/:id", get(contact_by_name))
        .route("/api/contacts/new", post(new_contact))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn index() -> String {
    String::from("Hello World")
}

async fn contacts(State(database): State<Arc<Database>>) -> impl IntoResponse {
    match database.select_all_contacts_from_database().await {
        Ok(contacts) => Json(contacts).into_response(),
        Err(msg) => (StatusCode::NO_CONTENT, format!("ERROR: {}", msg)).into_response(),
    }
}

async fn new_contact(
    extract::Json(payload): extract::Json<Contact>, // State(database): State<Arc<Database>>,
) {
}

async fn contact_by_id(
    State(database): State<Arc<Database>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match database.select_contact_by_id(id).await {
        Ok(contact) => {
            return Json(contact).into_response();
        }
        Err(msg) => {
            return (StatusCode::BAD_REQUEST, format!("ERROR: {}", msg)).into_response();
        }
    }
}

async fn contact_by_name(
    State(database): State<Arc<Database>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match database.select_contact_by_name(id).await {
        Ok(contact) => {
            return Json(contact).into_response();
        }
        Err(msg) => {
            return (StatusCode::BAD_REQUEST, format!("ERROR: {}", msg)).into_response();
        }
    }
}
