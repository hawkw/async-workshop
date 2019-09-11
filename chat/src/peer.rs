use tokio::{
    codec::{FramedWrite, LinesCodec},
    sync::mpsc,
    io::AsyncWrite,
};

use futures::{Sink, SinkExt, StreamExt};
use tracing::{debug, trace};
use std::fmt;

pub struct Forward {
    rx: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug)]
pub struct Peer {
    is_connected: bool,
    tx: mpsc::UnboundedSender<String>,
}

impl Peer {
    pub fn new() -> (Self, Forward) {
        let (tx, rx) = mpsc::unbounded_channel();
        let peer = Self {
            tx,
            is_connected: true,
        };
        let forward = Forward { rx };
        (peer, forward)
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

impl Forward {
    pub async fn forward_to<W>(mut self, write: W)
    where
        W: AsyncWrite + Unpin,
        FramedWrite<W, LinesCodec>: Sink<String>,
        <FramedWrite<W, LinesCodec> as Sink<String>>::Error: fmt::Display,
    {
        let mut lines = FramedWrite::new(write, LinesCodec::new());

        while let Some(msg) = self.rx.next().await {
            trace!(msg = msg.as_str(), "forwarding...");
            match lines.send(msg).await {
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
