use tokio::{
    codec::{FramedRead, FramedWrite, LinesCodec},
    net::{
        tcp::split::{TcpStreamReadHalf, TcpStreamWriteHalf},
        TcpStream,
    },
    sync::{mpsc, Lock},
};

use futures::{Poll, SinkExt, Stream, StreamExt};
use std::{collections::HashMap, error::Error, net::SocketAddr};
use tracing::{debug, trace};

pub struct PeerForward {
    io: FramedWrite<TcpStreamWriteHalf, LinesCodec>,
    rx: Receiver,
}

#[derive(Debug)]
pub struct Peer {
    is_connected: bool,
    tx: Sender,
}

pub type Sender = mpsc::UnboundedSender<String>;
pub type Receiver = mpsc::UnboundedReceiver<String>;

impl Peer {
    pub fn new(tx: Sender) -> Self {
        Self {
            tx,
            is_connected: true,
        }
    }

    pub async fn send(&mut self, msg: String) {
        if let Err(error) = self.tx.send(msg).await {
            self.is_connected = false;
            debug!(%error);
        }
        trace!(peer.is_connected = self.is_connected)
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}

impl PeerForward {
    pub fn new(write: TcpStreamWriteHalf, rx: mpsc::UnboundedReceiver<String>) -> Self {
        Self {
            io: FramedWrite::new(write, LinesCodec::new()),
            rx,
        }
    }

    pub async fn forward(mut self) {
        while let Some(msg) = self.rx.next().await {
            trace!(msg = msg.as_str(), "forwarding...");
            match self.io.send(msg).await {
                Ok(_) => {}
                Err(error) => {
                    debug!(%error, "error sending to peer");
                    return;
                }
            }
        }

        // The peer has disconnected, we can stop forwarding.
        debug!("peer disconnected");
    }
}
