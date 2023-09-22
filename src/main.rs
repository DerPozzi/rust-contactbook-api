use anyhow::Error;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Router;
use axum::{extract, Json};
use dotenv::dotenv;
use modules::contact;
use modules::database::Database;
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::{Pool, Postgres};
use std::ops::Deref;
use std::sync::Arc;
use std::{env, net::SocketAddr, time::Duration};

use tokio::sync::Mutex;
use tower::ServiceBuilder;

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
        .route(
            "/api/contacts/id/:id",
            get(contact_by_id).delete(delete_by_id),
        )
        .route("/api/contacts/name/:name", get(contact_by_name))
        .route(
            "/api/contacts/new",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| new_contact(body, shared_state)
            }),
        )
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
    extract::Json(payload): extract::Json<Contact>,
    database: Arc<Database>,
) -> impl IntoResponse {
    // Füge den neuen Kontakt zur Datenbank hinzu
    let result = database.insert_new_contact_into_database(payload).await;

    // Überprüfe, ob das Einfügen erfolgreich war
    match result {
        Ok(_) => StatusCode::CREATED.into_response(), // Wenn erfolgreich, gebe den Status "Created" zurück
        Err(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("ERROR: {}", msg)).into_response(), // Wenn ein Fehler auftritt, gebe den Status "Internal Server Error" zurück
    }
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
    Path(name): Path<String>,
) -> impl IntoResponse {
    match database.select_contact_by_name(name).await {
        Ok(contact) => {
            return Json(contact).into_response();
        }
        Err(msg) => {
            return (StatusCode::BAD_REQUEST, format!("ERROR: {}", msg)).into_response();
        }
    }
}

async fn delete_by_id(
    Path(id): Path<i64>,
    State(database): State<Arc<Database>>,
) -> impl IntoResponse {
    match database.delete_contact_by_id(id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(msg) => (StatusCode::BAD_REQUEST, format!("ERROR: {}", msg)).into_response(),
    }
}
