use std::collections::HashMap;

use uuid::Uuid;

use crate::packet::Packet;
use crate::peer::Peer;

#[derive(Debug, Default)]
pub struct Peers {
    map: HashMap<Uuid, Peer>,
}

impl Peers {
    #[inline]
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Peer> {
        self.map.get_mut(id)
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.map.keys().copied()
    }

    #[inline]
    pub fn insert(&mut self, id: Uuid, peer: Peer) -> Option<Peer> {
        self.map.insert(id, peer)
    }

    #[inline]
    pub async fn remove(&mut self, id: &Uuid) -> Option<Peer> {
        let peer = self.map.remove(id);
        match peer {
            Some(mut peer) => {
                peer.disconnect().await;
                Some(peer)
            }

            None => peer,
        }
    }

    pub async fn broadcast(&mut self, packet: Packet) {
        let sender = packet.id;
        for (id, peer) in self.map.iter_mut() {
            if *id == sender {
                continue;
            }

            peer.send(packet).await;
        }
    }
}