use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::{info, trace_span};
use tracing_futures::Instrument;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    FmtSubscriber,
};

mod peer;
mod server;

/// Tracing is a framework for adding structured, async-aware instrumentation
/// to Rust programs. This method sets up a tracing `Subscriber` to log traces
/// emitted by the server to the console.
fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::from_default_env()
        .add_directive(LevelFilter::INFO.into());
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing()?;

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
