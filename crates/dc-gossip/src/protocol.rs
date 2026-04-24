use crate::ping::Ping;
use anyhow::Result;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use solana_bloom::bloom::{Bloom, ConcurrentBloom};
use solana_hash::Hash;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CrdsFilter {
    pub filter: Bloom<Hash>,
    pub mask: u64,
    pub mask_bits: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrdsValue {
    pub label: String,
    pub pubkey: Pubkey,
    pub data: CrdsData,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrdsData {
    ContactInfo {
        gossip: String,
        tvu: String,
        tpu: String,
    },
    Vote(u64),
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub from: Pubkey,
    pub caller: CrdsValue,
    pub known: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullResponse {
    pub from: Pubkey,
    pub values: Vec<CrdsValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Protocol {
    PullRequest(CrdsFilter, CrdsValue),
    PullResponse(Pubkey, Vec<CrdsValue>),
    PushMessage(Pubkey, Vec<CrdsValue>),
    PingMessage(Ping),
    PongMessage(Ping),
    Unknown,
}

impl Protocol {
    pub fn encode_to(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }

    pub fn decode_from(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(bytes)?)
    }
}
