use actix_web::{post, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BodyType {
    email: String,
    password: String,
}

#[post("/register")]
async fn service(data: web::Json<BodyType>) -> impl Responder {
    HttpResponse::Ok().json(data)
}
