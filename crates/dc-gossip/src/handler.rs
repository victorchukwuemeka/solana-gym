use crate::crds::CrdsTable;
use crate::emitter::GossipTx;
use crate::protocol::Protocol;
use anyhow::Result;
use std::net::SocketAddr;

pub async fn handler(
    sender: SocketAddr,
    msg: Protocol,
    table: &mut CrdsTable,
    tx: &GossipTx,
) -> Result<()> {
    match msg {
        Protocol::PullResponse { from, value } => {
            tracing::info!("PullMessage from {}", from);
        }
        Protocol::PushMessage { from, value } => {
            tracing::info!("PushMessage from {}", from);
        }
        Protocol::PingMessage { from, value } => {
            tracing::info!("PingMessage from {}", from);
        }

        _ => {}
    }
    Ok(())
}
