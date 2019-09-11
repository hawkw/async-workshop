use structopt::StructOpt;

use crossterm_style::{Colorize, Styler};
use futures::{stream, Stream, Sink, SinkExt, StreamExt, TryStream, TryStreamExt};
use std::net::SocketAddr;
use tokio::{
    codec::{FramedRead, FramedWrite, LinesCodec, LinesCodecError},
    net::TcpStream,
};

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

type Error = Box<dyn std::error::Error>;

fn lines_from_conn(conn: TcpStream) -> (impl Stream<Item = Result<String, Error>>, impl Sink<String, Error = Error>) {
    let (read, write) = conn.split();

    let write_lines = FramedWrite::new(write, LinesCodec::new()).sink_err_into();
    let read_lines = FramedRead::new(read, LinesCodec::new()).err_into();
    (read_lines, write_lines)
}

fn lines_from_stdin() -> impl Stream<Item = Result<String, Error>> {
    let stdin = tokio::io::stdin();
    FramedRead::new(stdin, LinesCodec::new()).err_into()
}

#[derive(Debug)]
enum Event {
    Received(String),
    Input(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Options { server, user } = Options::from_args();

    let in_char = ">".bold().blue();

    let conn = TcpStream::connect(server).await?;
    println!("{} connected to {}", in_char, server);
    let (recv_lines, mut send_lines) = lines_from_conn(conn);

    send_lines.send(user).await?;

    let stdin_lines = lines_from_stdin().map_ok(Event::Input);
    let recv_lines = recv_lines.map_ok(Event::Received);
    let mut events = stream::select(stdin_lines, recv_lines);

    while let Some(event) = events.try_next().await? {
        match event {
            Event::Received(s) => println!("{} {}", in_char, s),
            Event::Input(s) => send_lines.send(s).await?,
        }
    }
    Ok(())
}
