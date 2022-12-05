use actix_web::{web, HttpResponse, Responder, Scope};

pub fn health_check_scope() -> Scope {
    web::scope("/health_check").service(health_check)
}

#[tracing::instrument(name = "Probe server health")]
#[actix_web::get("")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
