use std::net::TcpListener;

use api::configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::Settings::new().expect("Failed to retrieve settings.");

    let listener = TcpListener::bind(configuration.app.address())?;
    api::server::run(listener)?.await
}
