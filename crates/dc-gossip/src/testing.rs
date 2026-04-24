use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature, signer::keypair::Keypair, signer::Signer};

const PING_TOKEN_SIZE: usize = 32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ping {
    pub from: Pubkey,
    pub token: [u8; PING_TOKEN_SIZE],
    pub signature: Signature,
}

impl Ping {
    pub fn new(keypair: &Keypair) -> Result<Self> {
        // generate random token
        let token: [u8; PING_TOKEN_SIZE] = rand::thread_rng().gen();

        // sign the token
        let signature = keypair.sign_message(&token);

        Ok(Self {
            from: keypair.pubkey(),
            token,
            signature,
        })
    }
}
