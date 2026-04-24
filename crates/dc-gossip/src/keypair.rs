use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;

pub struct NodeKeypair {
    pub keypair: Keypair,
}

impl NodeKeypair {
    pub fn new() -> Self {
        Self {
            keypair: Keypair::new(),
        }
    }

    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
}
