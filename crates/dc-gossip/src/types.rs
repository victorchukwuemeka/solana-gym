//! Core types for dc-gossip.
//! Every other module imports from here.

use std::net::SocketAddr;

/// A validator discovered on the Solana network
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    pub id: String,               // validator identity pubkey
    pub gossip_addr: SocketAddr,  // their gossip port
    pub tvu_addr: Option<SocketAddr>, // where to receive shreds
    pub tpu_addr: Option<SocketAddr>, // where to send transactions
    pub last_seen: u64,           // unix timestamp
    pub version: u64,             // CRDS version for dedup
}

/// A slot announcement from the network
#[derive(Debug, Clone)]
pub struct SlotInfo {
    pub slot: u64,
    pub parent: u64,
    pub root: u64,
    pub validator_id: String,
}

/// Periodic cluster health snapshot
#[derive(Debug, Clone)]
pub struct ClusterHealth {
    pub active_validators: usize,
    pub latest_slot: u64,
}
