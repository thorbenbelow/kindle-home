use std::net::TcpListener;

use actix_web::{middleware::NormalizePath, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::routes::health_check;

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(NormalizePath::trim())
            .service(health_check::health_check_scope())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
