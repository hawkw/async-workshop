
use structopt::StructOpt;
use std::{
    fmt,
    net::SocketAddr,
    str::FromStr,
};

use tokio::{
    net::{
        TcpStream,
    },
    codec::{FramedRead, FramedWrite, LinesCodec},
};
use futures::{stream, StreamExt, TryStreamExt, SinkExt, FutureExt};
use crossterm_style::{Colorize, Styler};

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

#[derive(Debug)]
enum Event {
    Recieved(String),
    Input(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Options { server, user } = Options::from_args();

    let in_char = ">".bold().blue();

    let conn = TcpStream::connect(server).await?;
    println!("{} connected to {}", in_char, server, );

    let (read, write) = conn.split();

    let mut send_lines = FramedWrite::new(write, LinesCodec::new());
    send_lines.send(user).await?;

    let stdin = tokio::io::stdin();
    let stdin_lines = FramedRead::new(stdin, LinesCodec::new())
        .map_ok(Event::Input);
    let recv_lines = FramedRead::new(read, LinesCodec::new())
        .map_ok(Event::Recieved);
    let mut events = stream::select(stdin_lines, recv_lines);

    while let Some(event) = events.next().await {
        match event? {
            Event::Recieved(s) => println!("{} {}", in_char, s),
            Event::Input(s) => send_lines.send(s).await?,
        }
    }
    Ok(())
}
