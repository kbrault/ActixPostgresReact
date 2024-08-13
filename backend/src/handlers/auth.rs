use crate::config;
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

pub async fn login(user_data: web::Json<LoginRequest>) -> impl Responder {
    let conn_str = config::get_database_url();

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let stmt = client
        .prepare("SELECT password FROM users WHERE name ILIKE $1")
        .await
        .unwrap();
    let rows = client.query(&stmt, &[&user_data.login]).await.unwrap();

    if rows.is_empty() {
        return HttpResponse::Unauthorized().body("Invalid login or password");
    }

    let stored_password: String = rows[0].get("password");

    if verify(&user_data.password, &stored_password).unwrap() {
        HttpResponse::Ok().body("Login successful")
        // handle jwt
    } else {
        HttpResponse::Unauthorized().body("Invalid login or password")
    }
}
