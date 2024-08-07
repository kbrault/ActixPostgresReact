use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

async fn get_data() -> impl Responder {
    let sample_data = json!({
        "name": "John Doe",
        "age": 30,
        "email": "john.doe@example.com"
    });

    // Return JSON response
    HttpResponse::Ok().json(sample_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/data", web::get().to(get_data))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
