use crate::services::hue_client::{HueClient, HueClientSettings};
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder, Scope,
};

#[tracing::instrument(name = "Scope::Init::hue", skip(hue_config))]
pub fn hue_scope(hue_config: HueClientSettings) -> Scope {
    let hue_client = Data::new(HueClient::new(hue_config));
    web::scope("/hue")
        .app_data(hue_client.clone())
        .service(get_lights)
}

#[actix_web::get("/lights")]
async fn get_lights(_hue: web::Data<HueClient>) -> impl Responder {
    HttpResponse::Ok().finish()
}