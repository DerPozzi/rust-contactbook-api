use std::fmt::format;

use postgres::{Client, Error, NoTls};

use super::contact::Contact;

pub fn connect_to_database(url: &String) -> Result<Client, Error> {
    let client = Client::connect(url, NoTls)?;
    Ok(client)
}

pub fn create_table(client: &mut Client) -> Result<(), Error> {
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS contacts (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            birthday         VARCHAR,
            phone           VARCHAR,
            email           VARCHAR,
            notes           VARCHAR
            )
    ",
    )
}

pub fn insert_new_contact_into_database(
    client: &mut Client,
    contact: Contact,
) -> Result<u64, Error> {
    let name = contact.name;
    let birthday = contact.birthday.unwrap_or_default();
    let phone = contact.phone.unwrap_or_default();
    let email = contact.email.unwrap_or_default();
    let notes = contact.notes.unwrap_or_default();
    client.execute(
        "INSERT INTO contacts (name, birthday, phone, email, notes) VALUES ($1, $2, $3, $4, $5)",
        &[&name, &birthday, &phone, &email, &notes],
    )
}

pub fn select_contacts_from_database(
    client: &mut Client,
    name: String,
) -> Result<Vec<postgres::Row>, Error> {
    let name = format!("{}%", name);
    client.query(
        "SELECT * FROM contacts WHERE LOWER(name) LIKE LOWER($1) ORDER BY name",
        &[&name],
    )
}

pub fn select_all_contacts_from_database(client: &mut Client) -> Result<Vec<postgres::Row>, Error> {
    client.query("SELECT * FROM contacts", &[])
}
