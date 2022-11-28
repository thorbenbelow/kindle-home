use std::net::TcpListener;
use tokio::spawn;

const LOCALHOST: &'static str = "127.0.0.1";

pub struct TestApp {
    address: String,
}

impl TestApp {
    pub fn new() -> Self {
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
