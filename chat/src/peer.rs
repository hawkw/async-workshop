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
        self.is_connected = self.tx.send(msg.to_owned()).await.is_ok();
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
            match self.io.send(msg).await {
                Ok(_) => {}
                Err(error) => {
                    tracing::warn!(%error, "error sending to peer");
                    return;
                }
            }
        }
        // The peer has disconnected, we can stop forwarding.
    }
}
