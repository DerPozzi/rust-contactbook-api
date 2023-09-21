use dotenv::dotenv;
use postgres::{Client, Error, NoTls};
use std::{env, time::Duration};



fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("ERROR: Expected \"DB_URL\"");

    let mut client = conncet_to_database(&db_url)?;

    if client.is_valid(Duration::new(5, 0)).is_ok() {
        println!("Connection to DB successfull!")
    }

    Ok(())
}

fn conncet_to_database(url: &String) -> Result<Client, Error> {
    let client = Client::connect(url, NoTls)?;
    Ok(client)
}
