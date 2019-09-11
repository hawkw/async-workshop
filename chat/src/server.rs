use tokio::{
    codec::{FramedRead, FramedWrite, LinesCodec},
    net::{
        tcp::split::{TcpStreamReadHalf, TcpStreamWriteHalf},
        TcpStream,
    },
    sync::{mpsc, Lock},
};

use futures::{Poll, SinkExt, Stream, StreamExt};
use std::{collections::HashMap, net::SocketAddr};

use super::peer::{Peer, PeerForward};

#[derive(Debug, Clone)]
pub struct Server {
    peers: Lock<Peers>,
}

type Peers = HashMap<SocketAddr, Peer>;

impl Server {
    pub fn new() -> Self {
        Self {
            peers: Lock::new(Peers::new()),
        }
    }

    pub async fn serve_connection(mut self, connection: TcpStream, addr: SocketAddr) {
        let (read, write) = connection.split();
        let mut read_lines = FramedRead::new(read, LinesCodec::new());

        // The first line recieved from the peer is that peer's username.
        let name = match read_lines.next().await {
            Some(Ok(name)) => name,
            Some(Err(error)) => {
                tracing::warn!(peer.addr = %addr, %error, "an error occurred before the peer sent a username");
                return;
            }
            None => {
                tracing::warn!(peer.addr = %addr, "peer disconnected without sending a username");
                return;
            }
        };

        let rx = self.add_peer(addr).await;
        tracing::debug!(peer.addr = %addr, peer.name = %name, "peer connected");

        self.broadcast(addr, format!("{} ({}) joined the chat!", name, addr))
            .await;

        tokio::spawn(PeerForward::new(write, rx).forward());

        while let Some(result) = read_lines.next().await {
            match result {
                Ok(message) => self.broadcast(addr, format!("{}: {}", name, message)).await,
                Err(error) => {
                    tracing::warn!(%error, peer.name = %name, "peer disconnected");
                }
            }
        }

        self.broadcast(addr, format!("{} ({}) left the chat!", name, addr))
            .await;

        {
            let mut peers = self.peers.lock().await;
            peers.remove(&addr);
        }
    }

    #[tracing::instrument]
    async fn add_peer(&mut self, addr: SocketAddr) -> mpsc::UnboundedReceiver<String> {
        let mut peers = self.peers.lock().await;
        let (tx, rx) = mpsc::unbounded_channel();
        peers.insert(addr, Peer::new(tx));
        rx
    }

    #[tracing::instrument]
    async fn broadcast(&mut self, from: SocketAddr, msg: String) {
        tracing::debug!("broadcasting...");

        let mut peers = self.peers.lock().await;
        for (&addr, peer) in peers.iter_mut() {
            if addr == from {
                continue;
            }
            peer.send(msg.clone()).await;

            tracing::trace!(peer.addr = %addr, peer.is_connected = peer.is_connected());
        }
        peers.retain(|_, peer| peer.is_connected());
    }
}
