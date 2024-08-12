use crate::config;
use crate::db::initialize_db;
use actix_web::{HttpResponse, Responder};
//use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::NoTls;

/*
fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

async fn check_password(client: &Client, email: &str, password: &str) -> Result<bool, String> {
    // Query to get the hashed password for the given email
    let row = client
        .query_opt("SELECT password FROM users WHERE email = $1", &[&email])
        .await
        .map_err(|e| e.to_string())?;

    if let Some(row) = row {
        let stored_hash: &str = row.get(0);

        // Verify the provided password against the stored hash
        verify_password(password, stored_hash).map_err(|e| e.to_string())
    } else {
        // No user found with the given email
        Ok(false)
    }
}
    */

pub async fn get_data() -> impl Responder {
    let conn_str = config::get_database_url();

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    initialize_db(&client).await.unwrap();

    sleep(Duration::from_secs(1)).await;

    // Query the database
    let rows = client
        .query("SELECT name, age, email FROM users", &[])
        .await
        .unwrap();

    let mut data = Vec::new();
    for row in rows {
        let name: &str = row.get(0);
        let age: i32 = row.get(1);
        let email: &str = row.get(2);

        let row_data = json!({
            "name": name,
            "age": age,
            "email": email
        });

        data.push(row_data);
    }

    HttpResponse::Ok().json(data)
}
