use futures::prelude::*;
use tokio::{
    codec::{FramedRead, FramedWrite, LinesCodec},
    net::TcpStream,
    io,
};

pub type Error = Box<dyn std::error::Error>;

pub async fn run_client(user: String, mut stream: TcpStream) -> Result<(), Error> {
    // Split the raw `TcpStream` of bytes into a `Stream` of received lines and
    // a sink that we can write lines to send to.
    let (recv_lines, mut send_lines) = lines_from_conn(conn);

    // Start by sending the server our username.
    send_lines.send(user).await?;

    // Now you try! To finish implementing the client, we'll need to
    // continuously read from stdin and from `recv_lines`. When we receive a
    // line from stdin, we'll need to write that line to `send_lines`. When we
    // receive a line from `read_lines`, we need to print that to stdout.
    unimplemented!()
}

/// Splits a `TcpStream` of raw bytes into a `Stream` of lines and a sink that
/// we can write lines to.
fn lines_from_conn(conn: TcpStream) -> (impl Stream<Item = Result<String, Error>>, impl Sink<String, Error = Error>) {
    // Split `conn` into a read half and a write half, implementing the
    // AsyncRead trait and the AsyncWrite trait, respectively.
    let (read, write) = io::split(conn);

    // `FramedRead` turns an `AsyncRead` into a `Stream` of _frames_ using a
    // codec. When we read from the stream, the codec will asynchronously read
    // from the `AsyncRead` into a buffer until it reads a newline, and then
    // return the buffered frame.
    let read_lines = FramedRead::new(read, LinesCodec::new()).err_into();
    // Similarly, a `FramedWrite` implements a `Sink` for a frame type by
    // encoding each frame into bytes and writing those bytes to an
    // `AsyncWrite`.
    let write_lines = FramedWrite::new(write, LinesCodec::new()).sink_err_into();

    (read_lines, write_lines)
}

/// Returns a `Stream` of lines read from stdin.
fn lines_from_stdin() -> impl Stream<Item = Result<String, Error>> {
    let stdin = tokio::io::stdin();
    FramedRead::new(stdin, LinesCodec::new()).err_into()
}
