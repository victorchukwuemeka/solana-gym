use tracing::{info, warn, error, debug};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod gossipf;
mod rpc;
mod cli;
mod network;
mod ledger;
mod db;
mod poh;
mod gossip;

use ledger::Ledger;


use clap::Parser;


use cli::{Cli, Commands, GossipCommands};
use network::Network;


#[tokio::main]
async fn main()->anyhow::Result<()>{
    //let mut network = Network::Mainnet;
    
    //open or create a file for logging
    let file_appender = tracing_appender::rolling::never(".", "dummy_client.log");
    let (non_blocking,_guard) = tracing_appender::non_blocking(file_appender);
    
    //define the layers
    let stdout_layer = fmt::layer().with_target(false);
    let file_layer = fmt::layer().with_writer(non_blocking);


    tracing_subscriber::registry().with(stdout_layer)
    .with(file_layer)
    .with(EnvFilter::new("info"))
    .init();
    

    println!("Dummy client started");
    debug!("This is a debug log (hidden unless you set DEBUG filter)");
    
    gossipf::start().await?;
    //let peers = rpc::fetch_gossip().await?;
    //print!("Found peers from {}", peers.len());


    let cli = Cli::parse();
    match cli.command{
        Commands::Gossip { gossip_cmd,network ,url }=>{
            match gossip_cmd {
                GossipCommands::Peers => {
                    match rpc::fetch_gossip(network, url).await {
                        Ok(peers) => {
                            println!("Found {} peers", peers.len());
                            // You can iterate over peers here if needed
                            for peer in peers {
                                println!("Peer: {}", peer.pubkey);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching peers: {}", e);
                        }
                    }
                },
                GossipCommands::Slots => {
                    println!("Slot announcements not implemented yet");
                },

                GossipCommands::Tpu => {
                    let peers = rpc::fetch_gossip(network, url).await?;
                    for peer in peers.iter().filter(|p|p.tpu.is_some()) {
                        println!("Peer {} → TPU: {:?}", peer.pubkey, peer.tpu);
                    }

                }

                GossipCommands::Rpc=>{
                    let peers = rpc::fetch_gossip(network, url).await?;
                    for peer in peers.iter().filter(|p|p.rpc.is_some()){
                        println!("Peer {} -> RPC: {:?}", peer.pubkey, peer.rpc);
                    }
                }

                GossipCommands::All=>{
                     let peers = rpc::fetch_gossip(network, url).await?;
                    for peer in peers.iter().take(5){
                        println!(
                            "Peer: {} | Gossip: {:?} |Tpu: {:?} | RPC: {:?}",
                            peer.pubkey, peer.gossip, peer.tpu , peer.rpc
                        );
                    }  
                }
            }
        }
    }
    

    Ok(())
}