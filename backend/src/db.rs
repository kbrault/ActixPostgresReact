use thiserror::Error;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio_postgres::{Client, Error as PgError};

#[derive(Error, Debug)]
pub enum InitError {
    #[error("database error")]
    DbError(#[from] PgError),

    #[error("IO error")]
    IoError(#[from] io::Error),
}

/// Initializes the database, creating the users table if it does not exist.
pub async fn initialize_db(client: &Client) -> Result<(), InitError> {
    let table_exists: Option<Option<String>> = client
        .query("SELECT to_regclass('public.users')::text", &[])
        .await?
        .get(0)
        .and_then(|row| row.get(0))
        .map(|r: Option<String>| Some(r))
        .unwrap_or(None);

    if table_exists.is_none() {
        // Read the SQL file
        let sql = read_sql_file("src/init.sql").await?;

        // Execute the SQL statements
        client.batch_execute(&sql).await?;
    }

    Ok(())
}

/// Reads the SQL file asynchronously and returns the content as a String.
async fn read_sql_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
