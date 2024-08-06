use actix_web::{post, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BodyType {
    message: String,
}

#[post("/echo")]
async fn service(data: web::Json<BodyType>) -> impl Responder {
    println!("Received message: {}", data.message);
    HttpResponse::Ok().json(data)
}
