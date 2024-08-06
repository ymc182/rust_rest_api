use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use routes::{api::v1::echo, user::register};
use std::time::SystemTime;
mod error;
mod middleware;
mod routes;

#[get("/")]
async fn index() -> impl Responder {
    let now = SystemTime::now();
    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    HttpResponse::Ok().body(format!(
        "Server Running timestamp check:{}",
        timestamp.as_millis()
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api/v1")
                    .wrap(middleware::ApiKeyAuth::new("secret".to_string()))
                    .service(echo::service),
            )
            .service(web::scope("/user").service(register::service))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
