use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod config;
mod db;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/data", web::get().to(handlers::data::get_data))
            .route("/login", web::post().to(handlers::auth::login))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
