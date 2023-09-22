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
        let _:(i64,) = sqlx::query_as("INSERT INTO contacts (name, last_name, birthday, phone, email, notes) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&name)
            .bind(&last_name)
            .bind(birthday)
            .bind(phone)
            .bind(email)
            .bind(notes)
            .fetch_one(&self._pool)
            .await?;
        Ok(())
    }

    pub async fn select_all_contacts_from_database(&self) -> Result<Vec<Contact>, Error> {
        let contacts = sqlx::query_as::<_, Contact>("SELECT * FROM contacts ORDER BY name")
            .fetch_all(&self._pool)
            .await?;
        Ok(contacts)
    }
}
/*
pub fn select_contacts_by_name_from_database(
    client: &mut Client,
    name: String,
) -> Result<Vec<Contact>, Error> {
    let name = format!("{}%", name);
    let mut contacts: Vec<Contact> = Vec::new();
    for row in client.query(
        "SELECT * FROM contacts WHERE LOWER(name) LIKE LOWER($1) OR LOWER(last_name) LIKE LOWER($1) ORDER BY name",
        &[&name],
    )? {
        let contact = Contact { id: row.get(0), name: row.get(1), last_name: row.get(2), birthday: row.get(3), phone: row.get(4), email: row.get(5), notes: row.get(6) };
        contacts.push(contact);
    }

    Ok(contacts)
}

pub fn select_contact_by_id(client: &mut Client, id: i32) -> Result<Contact, Error> {
    let row = client.query_one("SELECT DISTINCT * FROM contacts WHERE id=$1", &[&id])?;
    Ok(Contact {
        id: row.get(0),
        name: row.get(1),
        last_name: row.get(2),
        birthday: row.get(3),
        phone: row.get(4),
        email: row.get(5),
        notes: row.get(6),
    })
}

pub async fn select_all_contacts_from_database(pool: PoolConnection<Postgres>) {
    let contacts = sqlx::query_as::<_, Contact>("SELECT * FROM contacts")
        .fetch_all(&pool)
        .await
        .unwrap();
}

pub fn edit_contact_by_id(client: &mut Client, contact: Contact) -> Result<u64, Error> {
    let id = contact.id.unwrap();
    let name = contact.name;
    let last_name = contact.last_name.unwrap_or_default();
    let birthday = contact.birthday.unwrap_or_default();
    let phone = contact.phone.unwrap_or_default();
    let email = contact.email.unwrap_or_default();
    let notes = contact.notes.unwrap_or_default();

    client.execute("UPDATE contacts SET name=$1, last_name=$2, birthday=$3, phone=$4, email=$5, notes=$6 WHERE id=$7", &[&name, &last_name, &birthday, &phone, &email, &notes, &id])
}

pub fn delete_contact_by_id(client: &mut Client, id: i32) -> Result<Vec<postgres::Row>, Error> {
    client.query("DELETE FROM contacts WHERE id=$1", &[&id])
}
*/
