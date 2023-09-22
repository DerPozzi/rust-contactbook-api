use anyhow::Error;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use axum::{http::StatusCode, Json};
use dotenv::dotenv;
use modules::database::{self, Database};
use sqlx::{postgres::PgPoolOptions, PgPool};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use std::{env, net::SocketAddr, time::Duration};

mod modules;

use crate::modules::contact::Contact;
use modules::api::{self, run_server};

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
        .route("/contacts/all", get(contacts))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn index() -> String {
    String::from("Hello World")
}

async fn contacts(State(database): State<Arc<Database>>) -> Json<Vec<Contact>> {
    let contacts = database
        .select_all_contacts_from_database()
        .await
        .expect("ERROR: Couldnt query all contacts");
    Json(contacts)
}

async fn new_contact(State(database): State<Arc<Database>>) {
    match database
        .insert_new_contact_into_database(Contact {
            id: None,
            name: "Gianluca".to_owned(),
            last_name: Some("Pozzo".to_owned()),
            birthday: None,
            phone: None,
            email: None,
            notes: None,
        })
        .await
    {
        Ok(_) => {}
        Err(msg) => println!("ERROR: {}", msg),
    }
}
