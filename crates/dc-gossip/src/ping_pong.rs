use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use solana_sdk::hash::Hash;
use solana_sdk::{pubkey::Pubkey, signature::Signature, signer::keypair::Keypair, signer::Signer};

const PING_TOKEN_SIZE: usize = 32;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ping {
    pub from: Pubkey,
    pub token: [u8; PING_TOKEN_SIZE],
    pub signature: Signature,
}

impl Ping {
    pub fn new(key: &Keypair) -> Result<Self> {
        let mut token = [0u8; PING_TOKEN_SIZE];
        rand::rng().fill_bytes(&mut token);
        let signature = key.sign_message(&token);

        Ok(Self {
            from: key.pubkey(),
            token,
            signature,
        })
    }
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct Pong {
    pub from: Pubkey,
    pub hash: Hash,
    pub signature: Signature,
}

impl Pong {
    pub fn new(ping: &Ping, keypair: &Keypair) -> Result<Self> {
        let mut buf = vec![];
        buf.extend_from_slice(b"SOLANA_PING_PONG");
        buf.extend_from_slice(&ping.token);
        let hash = solana_sdk::hash::hash(&buf);
        let signature = keypair.sign_message(hash.as_ref());
        Ok(Self {
            from: keypair.pubkey(),
            hash,
            signature,
        })
    }
}
