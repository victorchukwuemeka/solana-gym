use clap::{Parser,Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "dc", about = "Dummy Client CLI")]

pub struct Cli{
     #[command(subcommand)]
     pub command : Commands,
}


#[derive(Subcommand)]
pub enum Commands{
    Gossip{
        #[command(subcommand)]
        gossip_cmd: GossipCommands,

        #[arg(long, value_enum, default_value = "mainnet")]
        network: NetworkOpt,

        #[arg(long)]
        url: Option<String>,
    },

}


#[derive(Subcommand)]
pub enum GossipCommands {
    Peers,
    Slots,
    Tpu,
    Rpc,
    All
}


#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum NetworkOpt{
    Mainnet,
    Testnet,
    Devnet,
    Local,
    Custom
}