mod crds;
mod emitter;
mod handler;
mod keypair;
mod protocol;
mod transport;
mod types;

use anyhow::Result;
use crds::CrdsTable;
use emitter::create_channel;
use keypair::NodeKeypair;
use std::net::SocketAddr;
use tokio::net::lookup_host;
use transport::Transport;

const DEVNET_ENTRYPOINT: &str = "entrypoint.devnet.solana.com:8001";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let node = NodeKeypair::new();
    let transport = Transport::new("0.0.0.0:8901").await?;
    let mut table = CrdsTable::new();
    let (tx, _rx) = create_channel();

    tracing::info!("our node identity: {}", node.pubkey());
    tracing::info!("connecting to devnet: {}", DEVNET_ENTRYPOINT);
    //let entrypoint: SocketAddr = DEVNET_ENTRYPOINT.parse()?;
    let entrypoint = lookup_host(DEVNET_ENTRYPOINT)
        .await?
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not resolve devnet entrypoint"))?;

    let pull_response = protocol::Protocol::PullResponse {
        from: node.pubkey(),
        value: vec![],
    };

    let bytes = pull_response.encode_to()?;
    transport.send(&bytes, &entrypoint).await?;

    tracing::info!("pull request sent — listening for responses...");

    // replace the loop with this
    loop {
        match tokio::time::timeout(std::time::Duration::from_secs(5), transport.recv()).await {
            Ok(Ok((bytes, sender))) => {
                tracing::info!("packet from {} — {} bytes", sender, bytes.len());
                let msg = protocol::Protocol::decode_from(&bytes)?;
                handler::handler(sender, msg, &mut table, &tx).await?;
                tracing::info!("active validators: {}", table.get_peers().len());
            }
            Ok(Err(e)) => {
                tracing::error!("recv error: {}", e);
            }
            Err(_) => {
                tracing::info!("no packets in 5s — waiting...");
            }
        }
    }
}
