use crate::config;
use crate::db::initialize_db;
use actix_web::{HttpResponse, Responder};
use serde_json::json;
use tokio_postgres::NoTls;

pub async fn get_data() -> impl Responder {
    let conn_str = config::get_database_url();

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    initialize_db(&client).await.unwrap();

    let rows = client
        .query("SELECT name, role, email FROM users", &[])
        .await
        .unwrap();

    let mut data = Vec::new();
    for row in rows {
        let name: &str = row.get(0);
        let role: &str = row.get(1);
        let email: &str = row.get(2);

        let row_data = json!({
            "name": name,
            "role": role,
            "email": email
        });

        data.push(row_data);
    }

    HttpResponse::Ok().json(data)
}
