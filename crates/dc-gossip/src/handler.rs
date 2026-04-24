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
        Protocol::PushMessage(pubkey, values) => {
            tracing::info!("Push from {}", pubkey);
            // TODO: merge values into crds
        }
        Protocol::PullResponse(pubkey, values) => {
            tracing::info!("Pull response from {} — {} values", pubkey, values.len());
            // TODO: merge values into crds
        }
        Protocol::PingMessage(ping) => {
            tracing::info!("Ping from {}", ping.from);
            // TODO: send Pong back
        }
        _ => {
            tracing::info!("unknown message from {}", sender);
        }
    }
    Ok(())
}
