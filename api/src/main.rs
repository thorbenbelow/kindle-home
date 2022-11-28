use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000")?;
    api::server::run(listener)?.await
}
