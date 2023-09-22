use std::sync::Arc;

use anyhow::Error;
use sqlx::{pool::PoolConnection, Pool, Postgres};

use super::contact::{self, Contact};

pub struct Database {
    _pool: Pool<Postgres>,
}

impl Database {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Database { _pool: pool }
    }

    pub async fn close_connection(&self) {
        self._pool.close().await;
    }

    pub async fn create_table(&self) {
        if !self._pool.is_closed() {
            match sqlx::query(
                "CREATE TABLE IF NOT EXISTS contacts (
                id              SERIAL PRIMARY KEY,
                name            VARCHAR NOT NULL,
                last_name       VARCHAR,
                birthday         VARCHAR,
                phone           VARCHAR,
                email           VARCHAR,
                notes           VARCHAR
                )",
            )
            .fetch_all(&self._pool)
            .await
            {
                Ok(_) => return,
                Err(msg) => println!("ERROR: {}", msg),
            }
        }
    }
    pub async fn select_contact_by_id(&self, id: i64) -> Result<Contact, Error> {
        let contact = sqlx::query_as::<_, Contact>("SELECT * FROM contacts WHERE id=$1")
            .bind(id)
            .fetch_one(&self._pool)
            .await?;
        Ok(contact)
    }

    pub async fn select_contact_by_name(&self, id: String) -> Result<Contact, Error> {
        let contact = sqlx::query_as::<_, Contact>(
            "SELECT * FROM contacts WHERE LOWER(name) LIKE LOWER($1) ORDER BY name",
        )
        .bind(id)
        .fetch_one(&self._pool)
        .await?;
        Ok(contact)
    }

    pub async fn insert_new_contact_into_database(&self, contact: Contact) -> Result<(), Error> {
        let name = contact.name;
        let last_name = contact.last_name.unwrap_or_default();
        let birthday = contact.birthday.unwrap_or_default();
        let phone = contact.phone.unwrap_or_default();
        let email = contact.email.unwrap_or_default();
        let notes = contact.notes.unwrap_or_default();
        let _ = sqlx::query("INSERT INTO contacts (name, last_name, birthday, phone, email, notes) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&name)
            .bind(&last_name)
            .bind(birthday)
            .bind(phone)
            .bind(email)
            .bind(notes).execute(&self._pool).await?;
        Ok(())
    }

    pub async fn select_all_contacts_from_database(&self) -> Result<Vec<Contact>, Error> {
        let contacts = sqlx::query_as::<_, Contact>("SELECT * FROM contacts ORDER BY name")
            .fetch_all(&self._pool)
            .await?;
        Ok(contacts)
    }

    pub async fn delete_contact_by_id(&self, id: i64) -> Result<(), Error> {
        match sqlx::query_as::<_, Contact>("SELECT * FROM contacts WHERE id=$1")
            .bind(id)
            .fetch_one(&self._pool)
            .await
        {
            Ok(_) => {}
            Err(msg) => return Err(Error::msg(msg)),
        }

        let _ = sqlx::query("DELETE FROM contacts WHERE id=$1")
            .bind(id)
            .fetch_one(&self._pool)
            .await;
        Ok(())
    }
}
