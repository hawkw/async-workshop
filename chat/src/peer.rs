use tokio::{
    codec::{Framed, LinesCodec},
    net::TcpStream,
    sync::mpsc,
};

pub struct Peer {
    name: String,
    lines: Framed<TcpStream, LinesCodec>,
    server: server::Server,
    rx: Receiver,
}

pub type Sender = mpsc::UnboundedSender<String>;
pub type Receiver = mpsc::UnboundedReceiver<String>;

impl Peer {
    /// Create a new instance of `Peer`.
    async fn new(lines: Framed<TcpStream, LinesCodec>, server: server::Server) -> io::Result<Peer> {
        // The first line recieved from the peer is that peer's username.
        let name = lines.next().await?;

        let ip = lines.get_ref().peer_addr()?;

        let (tx, rx) = mpsc::unbounded();

        server.add_peer(ip, tx);

        Peer {
            name,
            lines,
            server,
            rx,
        }
    }
}
