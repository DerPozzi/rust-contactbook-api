use dotenv::dotenv;
use modules::database::{
    insert_new_contact_into_database, select_all_contacts_from_database,
    select_contacts_from_database,
};
use postgres::Error;
use std::{env, time::Duration};

mod modules;

use crate::modules::contact::Contact;
use crate::modules::database;

fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("ERROR: Expected \"DB_URL\"");

    let mut client = database::connect_to_database(&db_url)?;

    if client.is_valid(Duration::new(5, 0)).is_ok() {
        println!("Connection to DB successfull!")
    }

    database::create_table(&mut client)?;

    insert_new_contact_into_database(
        &mut client,
        Contact {
            name: "Gianluca".to_owned(),
            id: None,
            birthday: Some("99.99.9999".to_owned()),
            phone: Some("123456789".to_owned()),
            email: None,
            notes: None,
        },
    )?;
    insert_new_contact_into_database(
        &mut client,
        Contact {
            name: "Emily".to_owned(),
            id: None,
            birthday: Some("99.99.9999".to_owned()),
            phone: Some("0123456789".to_owned()),
            email: None,
            notes: Some("doofe nuss".to_owned()),
        },
    )?;

    for row in select_contacts_from_database(&mut client, "emily".to_owned())? {
        let contact = Contact {
            id: row.get(0),
            name: row.get(1),
            birthday: row.try_get(2).ok(),
            phone: row.try_get(3).ok(),
            email: row.try_get(4).ok(),
            notes: row.try_get(5).ok(),
        };
        println!("{:?}", contact);
        println!();
    }

    println!();

    for row in select_all_contacts_from_database(&mut client)? {
        println!("{:?}", row);
        println!();
    }

    client.execute("DROP TABLE contacts", &[])?;

    Ok(())
}
