use tokio::{
    codec::{FramedWrite, LinesCodec},
    sync::mpsc,
    io::AsyncWrite,
};

use tracing::{debug, trace};
use std::fmt;
use futures::prelude::*;

pub struct Forward {
    rx: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug)]
pub struct Peer {
    tx: mpsc::UnboundedSender<String>,
}

impl Peer {
    // Returns a new `Peer` handle to send messages to this peer, and a
    // `Forward` that forwards those messages on the peer's connection.
    pub fn new() -> (Self, Forward) {
        let (tx, rx) = mpsc::unbounded_channel();
        let peer = Self { tx, };
        let forward = Forward { rx };
        (peer, forward)
    }

    /// Send a message on this peer's channel.
    pub async fn send(&mut self, msg: String) {
        if let Err(error) = self.tx.send(msg).await {
            debug!(%error);
        }
    }
}

impl Forward {
    /// Forward each line recieved from the `Peer` to the provided AsyncWrite.
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
