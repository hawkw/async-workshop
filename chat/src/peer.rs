use tokio::{
    codec::{FramedWrite, LinesCodec},
    sync::mpsc,
    io::AsyncWrite,
};

use futures::{Sink, SinkExt, StreamExt};
use tracing::{debug, trace};
use std::fmt;

pub struct Forward<W> {
    io: FramedWrite<W, LinesCodec>,
    rx: Receiver,
}

#[derive(Debug)]
pub struct Peer {
    is_connected: bool,
    tx: Sender,
}

type Sender = mpsc::UnboundedSender<String>;
type Receiver = mpsc::UnboundedReceiver<String>;

impl Peer {
    pub fn new<W: AsyncWrite>(write: W) -> (Self, Forward<W>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let peer = Self {
            tx,
            is_connected: true,
        };
        let forward = Forward::new(write, rx);
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

impl<W: AsyncWrite> Forward<W> {
    fn new(write: W, rx: mpsc::UnboundedReceiver<String>) -> Self {
        Self {
            io: FramedWrite::new(write, LinesCodec::new()),
            rx,
        }
    }

    pub async fn forward(mut self)
    where
        W: Unpin,
        FramedWrite<W, LinesCodec>: Sink<String>,
        <FramedWrite<W, LinesCodec> as Sink<String>>::Error: fmt::Display,
    {
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
