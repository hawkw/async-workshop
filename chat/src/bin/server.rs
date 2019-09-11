use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::{info, trace_span};
use tracing_subscriber::{FmtSubscriber, filter::{Filter, LevelFilter}};
use tracing_futures::Instrument;

use chat::server;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_filter(Filter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let addr = "127.0.0.1:7000".parse::<SocketAddr>()?;
    let mut listener = TcpListener::bind(addr).await?;
    info!(local.addr = %addr, "listening on");

    let server = server::Server::new();

    loop {
        let (socket, addr) = listener.accept().await?;
        let serve = server
            .clone()
            .serve_connection(socket, addr)
            .instrument(trace_span!("serve", peer.addr = %addr));
        tokio::spawn(serve);
    }
}
