use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_gossip::crds_value::CrdsValue;
use solana_gossip::ping_pong::{Ping, Pong};
//use solana_gossip::protocol::Pong;
//use solana_gossip::protocol::Protocol as SolanaProtocol;

use solana_pubkey::Pubkey;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Protocol {
    PullResponse(Pubkey, Vec<CrdsValue>),
    PushMessage(Pubkey, Vec<CrdsValue>),
    PingMessage(Ping),
    PongMessage(Pong),
    Unknown,
}

impl Protocol {
    pub fn decode_from(bytes: &[u8]) -> Result<Self> {
        match bincode::deserialize(bytes) {
            Ok(msg) => Ok(msg),
            Err(_) => Ok(Protocol::Unknown),
        }
    }

    pub fn encode_to(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }
}
