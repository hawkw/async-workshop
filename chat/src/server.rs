use tokio::sync::{mpsc, Lock};

pub struct Server {
    peers: Lock<Peers>,
}

#[derive(Debug)]
pub struct Peers {
    peers: HashMap<String, peer::Sender>,
}

impl Server {
    pub async fn add_peer(&self, peer_addr: SocketAddr, sender: peer::Sender, name: &str) {
        let mut peers = self.peers.lock().await;
        peers.peers.insert(peer);
        let msg = format!("{} ({}) joined the chat!", name, peer_addr.ip());
        peers.send_all(peer_addr, &msg).await
    }
}

impl Peers {
    async fn send_all(&mut self, from: SocketAddr, message: &str) {
        peers.retain(|(addr, sender)| {
            if addr == from {
                return true;
            }

            sender.send(message.to_owned()).await.is_ok()
        })
    }
}
