use std::net::TcpListener;

use actix_web::{middleware::NormalizePath, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::configuration::Settings;
use crate::routes::health_check;
use crate::routes::hue;

pub fn run(
    configuration: Settings,
    listener: TcpListener,
) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .wrap(NormalizePath::trim())
            .service(health_check::health_check_scope())
            .service(hue::hue_scope(configuration.hue.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
