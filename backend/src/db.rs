use tokio_postgres::{Client, Error};

/// Initializes the database, creating the users table if it does not exist.
pub async fn initialize_db(client: &Client) -> Result<(), Error> {
    let table_exists: Option<Option<String>> = client
        .query("SELECT to_regclass('public.users')::text", &[])
        .await?
        .get(0)
        .and_then(|row| row.get(0))
        .map(|r: Option<String>| Some(r))
        .unwrap_or(None);

    if table_exists.is_none() {
        client
            .batch_execute(
                "
                CREATE TABLE users (
                    id SERIAL PRIMARY KEY,
                    name VARCHAR(100) NOT NULL,
                    age INT NOT NULL,
                    email VARCHAR(100) NOT NULL
                );
                INSERT INTO users (name, age, email) VALUES
                ('Alice', 30, 'alice@example.com'),
                ('Bob', 25, 'bob@example.com'),
                ('Charlie', 35, 'charlie@example.com');
                ",
            )
            .await?;
    }

    Ok(())
}
