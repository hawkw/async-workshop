#![feature(async_await)]
use chat::server;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::builder()
      .with_filter(tracing_subscriber::Filter::from_default_env()).finish());

    let mut listener = TcpListener::bind("127.0.0.1:7000").await?;
    println!("listening on port 7000");

    let server = server::Server::new();

    loop {
        let (socket, addr) = listener.accept().await?;
        let server = server.clone();
        tokio::spawn(server.serve_connection(socket, addr));
    }
}
