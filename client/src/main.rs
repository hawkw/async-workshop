use structopt::StructOpt;
use std::net::SocketAddr;
use tokio::net::TcpStream;

// Comment out or remove the `path` attribute when you're ready to start
// implementing the client!
#[path = "../solutions/client.rs"]
mod client;

#[derive(Debug, StructOpt)]
#[structopt(name = "client", about = "a simple chat client")]
struct Options {
    /// The name of the server to connect to.
    #[structopt(short, long, default_value = "127.0.0.1:7000")]
    server: SocketAddr,

    /// The username to connect as.
    #[structopt()]
    user: String,
}

// The `tokio::main` attribute allows us to use an `async fn` as `main()`.
// Behind the scenes, the attribute handles setting up a tokio runtime & running
// the provided async function on that runtime until it completes.
#[tokio::main]
async fn main() -> Result<(), client::Error> {
    // Parse command line options.
    let Options { server, user } = Options::from_args();

    // Try to connect to the server, returning a `TcpStream`.
    let conn = TcpStream::connect(server).await?;
    println!("connected to {}", server);

    client::run_client(user, conn).await
}
