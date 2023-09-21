use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::{env, time::Duration};

mod modules;

use crate::modules::contact::Contact;

fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("ERROR: Expected \"DB_URL\"");

    let mut client = connect_to_database(&db_url)?;

    if client.is_valid(Duration::new(5, 0)).is_ok() {
        println!("Connection to DB successfull!")
    }

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
    )?;

    client.execute(
        "INSERT INTO contacts (name, birthday) VALUES ($1, $2)",
        &[&"Gianluca", &"05.09.2003"],
    )?;

    let test = client.query("SELECT * FROM contacts", &[])?;

    for row in test {
        let contact = Contact {
            id: row.get(0),
            name: row.get(1),
            birthday: row.try_get(2).ok(),
            phone: row.try_get(3).ok(),
            email: row.try_get(4).ok(),
            notes: row.try_get(5).ok(),
        };
        println!("Hello {:?}", contact);
    }

    client.execute("DROP TABLE contacts", &[])?;

    Ok(())
}

fn connect_to_database(url: &String) -> Result<Client, Error> {
    let client = Client::connect(url, NoTls)?;
    Ok(client)
}
