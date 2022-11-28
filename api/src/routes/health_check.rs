use actix_web::{
    middleware::{self, NormalizePath},
    web::{self, to},
    HttpResponse, Responder, Scope,
};

pub fn health_check_scope() -> Scope {
    web::scope("/health_check").service(health_check)
}

#[actix_web::get("")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
