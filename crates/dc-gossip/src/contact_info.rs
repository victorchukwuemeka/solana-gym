use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_version::Version;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub pubkey: Pubkey,
    pub wallclock: u64,
    pub unset: u64,
    pub shred_version: u16,
    pub version: Version,
    pub addrs: Vec<IpAddr>,        // ← add this
    pub sockets: Vec<SocketEntry>, // ← add this
    pub extensions: Vec<Extension>,
}

impl ContactInfo {
    pub fn new(pubkey: Pubkey, gossip_addr: SocketAddr, shred_version: u16) -> Self {
        Self {
            pubkey,
            wallclock: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            gossip_addr,
            shred_version,
        }
    }
}
