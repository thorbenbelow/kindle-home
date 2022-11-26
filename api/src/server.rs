use std::net::TcpListener;

use actix_web::{App, HttpServer};

use crate::routes::health_check;

pub fn run(listener: TcpListener) -> Result<actix_web::dev::Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check::health_check))
        .listen(listener)?
        .run();
    Ok(server)
}
