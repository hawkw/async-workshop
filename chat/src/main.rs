#![feature(async-await)]
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::codec::Decoder;

mod peer;
mod server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut listener = TcpListener::bind("127.0.0.1:7000").await?;
  let mut
  println!("listening on port 7000");
  loop {
    let (socket, _) = listener.accept().await?;
  }
}
