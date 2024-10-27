use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
