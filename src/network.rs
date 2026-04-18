use crate::cli::NetworkOpt;

#[derive(Debug, Clone)]
pub enum Network{
    Mainnet,
    Testnet,
    Devnet,
    Localnet,
    Custom(String)
}


impl Network {
    pub fn rpc_url(&self)-> &str{
        match self {
            Network::Mainnet => "https://api.mainnet-beta.solana.com",
            Network::Testnet => "https://api.testnet.solana.com",
            Network::Devnet => "https://api.devnet.solana.com",
            Network::Localnet=> "http://127.0.0.1:8899",
            Network::Custom(url) => url.as_str(),
        }
    }
}

impl From<(NetworkOpt, Option<String>)> for Network {
    fn from((opt, url): (NetworkOpt, Option<String>)) -> Self {
        match opt{
            NetworkOpt::Mainnet => Network::Mainnet,
            NetworkOpt::Testnet => Network::Testnet,
            NetworkOpt::Devnet => Network::Devnet,
            NetworkOpt::Local =>  Network::Localnet,
            NetworkOpt::Custom => {
                if let Some(u) = url {
                    Network::Custom(u)
                } else {
                    panic!("--url is required when using --network custom");
                }
            },
        }
    }
}