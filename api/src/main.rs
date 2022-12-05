use std::net::TcpListener;

use api::configuration;
use api::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // read config
    let configuration = configuration::Settings::new().expect("Failed to read config.");

    // set up tracking
    let subscriber = get_subscriber("kindle_api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // start server
    let listener = TcpListener::bind(configuration.app.address())?;
    api::server::run(configuration, listener)?.await
}
