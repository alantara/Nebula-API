use actix_web::{get, Responder, HttpResponse};

#[get("/health")]
pub async fn get_health() -> impl Responder {
    HttpResponse::Ok()
}