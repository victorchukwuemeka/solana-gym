use crate::contact_info::ContactInfo;
use crate::ping_pong::Ping;
use crate::ping_pong::Pong;
use anyhow::Result;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use solana_bloom::bloom::{Bloom, ConcurrentBloom};
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

use solana_sdk::hash::Hash;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CrdsFilter {
    pub filter: Bloom<Hash>,
    pub mask: u64,
    pub mask_bits: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrdsValue {
    pub signature: Signature,
    pub data: CrdsData,
    #[serde(skip_serializing)]
    pub hash: Hash,
}

impl CrdsValue {
    pub fn new_contact_info(info: ContactInfo, keypair: &Keypair) -> Self {
        let data = CrdsData::ContactInfo(info);
        let bytes = bincode::serialize(&data).unwrap();
        let signature = keypair.sign_message(&bytes);
        let hash = solana_sdk::hash::hash(&bytes);
        Self {
            signature,
            data,
            hash,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CrdsData {
    LegacyContactInfo(Vec<u8>),         // 0 - deprecated, just bytes
    Vote(u8, Vec<u8>),                  // 1
    LowestSlot(u8, Vec<u8>),            // 2
    LegacySnapshotHashes(Vec<u8>),      // 3
    AccountsHashes(Vec<u8>),            // 4
    EpochSlots(u8, Vec<u8>),            // 5
    LegacyVersion(Vec<u8>),             // 6
    Version(Vec<u8>),                   // 7
    NodeInstance(Vec<u8>),              // 8
    DuplicateShred(u16, Vec<u8>),       // 9
    SnapshotHashes(Vec<u8>),            // 10
    ContactInfo(ContactInfo),           // 11 ← our real one
    RestartLastVotedForkSlots(Vec<u8>), // 12
    RestartHeaviestFork(Vec<u8>),       // 13
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
    PongMessage(Pong),
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
