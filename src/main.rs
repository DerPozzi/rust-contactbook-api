use dotenv::dotenv;
use postgres::{Client, Error};
use std::{env, net::SocketAddr, time::Duration};
use tokio::task;

mod modules;

use crate::modules::contact::Contact;
use modules::{
    api::run_server,
    database::{
        self, connect_to_database, delete_contact_by_id, insert_new_contact_into_database,
        select_all_contacts_from_database, select_contacts_by_name_from_database,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("ERROR: Expected \"DB_URL\"");

    let mut client = connect_to_database(db_url.as_str()).await?;

    if client.is_valid(Duration::new(5, 0)).is_ok() {
        println!("Connection to DB successfull!")
    }

    database::create_table(&mut client)?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tokio::spawn(async move {
        run_server(addr).await;
    });

    // client.close()?;

    Ok(())
}
