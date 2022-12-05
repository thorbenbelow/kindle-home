use std::net::TcpListener;
use tokio::spawn;
use once_cell::sync::Lazy;
use api::telemetry::{get_subscriber, init_subscriber};


static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    address: String,
}

impl TestApp {
    pub fn new() -> Self {
        Lazy::force(&TRACING);

        const LOCALHOST: &'static str = "127.0.0.1";
        let listener = TcpListener::bind(format!("{}:0", LOCALHOST)).expect("Failed to bind.");

        let port = listener
            .local_addr()
            .expect("Failed to retrieve port.")
            .port();

        let server = api::server::run(listener).expect("Failed to run app.");
        let _ = spawn(server);
        TestApp {
            address: format!("http://{}:{}", LOCALHOST, port),
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.address, path)
    }
}
