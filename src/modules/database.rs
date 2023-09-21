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
            last_name       VARCHAR,
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
    let last_name = contact.last_name.unwrap_or_default();
    let birthday = contact.birthday.unwrap_or_default();
    let phone = contact.phone.unwrap_or_default();
    let email = contact.email.unwrap_or_default();
    let notes = contact.notes.unwrap_or_default();
    client.execute(
        "INSERT INTO contacts (name, last_name, birthday, phone, email, notes) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&name, &last_name, &birthday, &phone, &email, &notes],
    )
}

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
    let row = client.query_one("SLECT DISTINCT * FROM contacts WHERE id=$1", &[&id])?;
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

pub fn select_all_contacts_from_database(client: &mut Client) -> Result<Vec<postgres::Row>, Error> {
    client.query("SELECT * FROM contacts", &[])
}

pub fn edit_contact_by_id(client: &mut Client, contact: Contact) -> Result<u64, Error> {
    let id = contact.id.unwrap();
    let name = contact.name;
    let last_name = contact.last_name.unwrap_or_default();
    let birthday = contact.birthday.unwrap_or_default();
    let phone = contact.phone.unwrap_or_default();
    let email = contact.email.unwrap_or_default();
    let notes = contact.notes.unwrap_or_default();

    client.execute("UPDATE contacts SET name=$1, last_name=$2, birthday=$3, phone=$4, email=$5, notes=$6 WHERE id=$6", &[&name, &last_name, &birthday, &phone, &email, &notes, &id])
}
