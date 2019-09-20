use std::net::SocketAddr;
use structopt::StructOpt;

use tokio::net::TcpListener;
use tracing::{info, trace_span};
use tracing_futures::Instrument;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    FmtSubscriber,
};

// Comment out these `path` attributes when you're ready to work on your own
// implementation!
#[path = "../solutions/peer.rs"]
mod peer;
#[path = "../solutions/server.rs"]
mod server;

#[derive(Debug, StructOpt)]
#[structopt(name = "server", about = "an asynchronous chat swerver")]
struct Options {
    /// The address to serve on
    #[structopt(short, long, default_value = "127.0.0.1:7000")]
    addr: SocketAddr,
}

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

    // Parse command line options and bind a socket on the provided address.
    let Options { addr } = Options::from_args();

    let mut listener = TcpListener::bind(addr).await?;
    info!(local.addr = %addr, "listening on");

    let server = server::Server::new();

    loop {
        let (socket, addr) = listener.accept().await?;
        // Clone `Arc` on the shared server state for each connection.
        let server = server.clone();
        // Call the `serve_connection` async method on server, returning a future.
        let serve = server
            .serve_connection(socket, addr)
            .instrument(trace_span!("serve", peer.addr = %addr));
        // Spawn the future for that connection onto the `tokio` runtime.
        tokio::spawn(serve);
    }
}
