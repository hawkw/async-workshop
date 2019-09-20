use tokio::{
    codec::{FramedRead, LinesCodec},
    net::TcpStream,
    io,
    sync::Mutex,
};

use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tracing::{debug, info, warn, trace_span};
use tracing_futures::Instrument;
use futures::prelude::*;

use super::peer::{self, Peer};

#[derive(Debug, Clone)]
pub struct Server {
    peers: Arc<Mutex<Peers>>,
}

type Peers = HashMap<SocketAddr, Peer>;

impl Server {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(Mutex::new(Peers::new())),
        }
    }

    pub async fn serve_connection(self, connection: TcpStream, addr: SocketAddr) {
        // Split the TcpStream into read and write halves.
        let (read, write) = io::split(connection);
        let mut read_lines = FramedRead::new(read, LinesCodec::new());

        // The first line recieved from the peer is that peer's username.
        let name = match read_lines.next().await {
            Some(Ok(name)) => name,
            // If the peer hung up or was disconnected before sending a
            // username, we're done!
            Some(Err(error)) => {
                warn!(%error, "an error occurred before the peer sent a username");
                return;
            }
            None => {
                info!("peer disconnected before sending a username");
                return;
            }
        };

        // Tell everyone that a new peer has joined the chat.
        let msg = format!("{} ({}) joined the chat!", name, addr);
        self.broadcast(addr, msg).await;
        debug!(peer.name = %name);

        // Insert the new peer into our map of peers,returning a handle that
        // forwards broadcasted messages to that peer.
        let forward = self.add_peer(addr).await;

        // Spawn a task in the background that continuously forwards messages we
        // broadcast to that peer.
        tokio::spawn(forward.forward_to(write));

        while let Some(result) = read_lines.next().await {
            match result {
                Ok(message) => {
                    let msg = format!("{}: {}", name, message);
                    self.broadcast(addr, msg).await;
                }
                Err(error) => {
                    debug!(%error, peer.name = %name);
                }
            }
        }

        // When the stream ends, the peer has disconnected. Remove it from the
        // map and let everyone else know.
        self.remove_peer(addr).await;
        let msg = format!("{} ({}) left the chat!", name, addr);
        self.broadcast(addr, msg).await;
    }

    /// Add a new peer to the server, returning a task that will forward
    async fn add_peer(&self, addr: SocketAddr) -> peer::Forward {
        let (peer, forward) = Peer::new();
        let mut peers = self.peers.lock().await;
        peers.insert(addr, peer);
        forward
    }

    /// Remove a peer from the server.
    async fn remove_peer(&self, addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        peers.remove(&addr);
    }

    /// Broadcast a message from the peer at address `from` to every other peer.
    #[tracing::instrument]
    async fn broadcast(&self, from: SocketAddr, msg: String) {
        debug!("broadcasting...");

        let mut peers = self.peers.lock().await;
        for (&addr, peer) in peers.iter_mut() {
            if addr == from {
                continue;
            }

            peer.send(msg.clone())
                .instrument(trace_span!("send_to", peer = %addr))
                .await;
        }
    }
}
