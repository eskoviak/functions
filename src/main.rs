use postgres::{Client, NoTls, Error};
use env::var;

fn main() -> Result<(), Error> {

    let pg_url = env:var("PG_LOCAL").unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/library".to_string());

    println!("Connecting to PostgreSQL at: {}", pg_url);

    /*
    let mut client = Client::connect(&pg_url, NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ")?;
    */
    Ok(())

}
