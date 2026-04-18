
use std::thread::sleep;
use std::time::SystemTimeError;
use anyhow::Result;
use anyhow::anyhow;
use std::time::Duration;


use crate::rpc::{fetch_gossip, GossipPeer};
use crate::network::Network;
use crate::cli::NetworkOpt;
use crate::db::Database;
use serde_json::Value;


pub struct Ledger{
    pub rpc_url : String,
    //pub db_path : String 
    pub db: Database,
}

#[derive(Debug)]
pub struct ParsedBlock{
    pub slot : u64,
    pub parent_slot : u64,
    pub blockhash : String,
    pub block_time: i64,
    pub transactions: Vec<ParsedTransaction> 
}

#[derive(Debug)]
pub struct ParsedTransaction{
    pub signature : String,
    pub signers : Vec<String>,
    pub program : String,
    pub status : String
}


//the design for the ledger is first get a peer from the validator via gossip
//fetch the block  aka fetch_peers 
//parse the block fechted aka blockparser
impl Ledger{
    pub fn new(rpc_url: String, db_path: String)->Self{
        let db = Database::new(db_path);
        Self { rpc_url, db }
    }

    //i'm adding the rpc for getting the ledger here .
    pub async  fn fetch_peers(&self, network_opt: NetworkOpt, custom_url: Option<String>)->Result<Vec<GossipPeer>>
    {
        let peers = fetch_gossip(network_opt, custom_url).await?;
        Ok(peers)
    }
   
    
    pub async fn fetch_block(&self, slot:u64)->Result<serde_json::Value>{
      
        let client  = reqwest::Client::new();
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlock",
            "params": [
                slot,
                { "encoding": "json", "transactionDetails": "full", "rewards": false }
            ]
        });

        let mut attempts = 0;
        loop{
            attempts += 1;
            
            let res = client.post(&self.rpc_url).json(&payload).send().await;
            
            match res {
                Ok(resp)=>{
                    let body: Value = resp.json().await?;
                    if !body["result"].is_null() {
                        return Ok(body["result"].clone())
                    }else {
                        return Err(anyhow!(
                            "No result field in response: {:?}",
                            body
                        ));
                    }
                }
                Err(e) =>{
                    if attempts >= 3 {
                        return Err(anyhow!("Failed to fetch block after 3 attempts: {}", e));
                    }
                    eprintln!("Error fetching block (attempt {}): {}. Retrying...", attempts, e);
                    sleep(Duration::from_secs(2_u64.pow(attempts)));
                }
            }
        }




    }

    pub fn parse_block(json : &Value)->Option<ParsedBlock>{
        let slot = json["slot"].as_u64()?;
        let parent_slot = json["parentSlot"].as_u64().unwrap_or(0);
        let blockhash = json["blockhash"].as_str().unwrap_or_default().to_string();
        let block_time = json["blockTime"].as_i64().unwrap_or(0);

        let mut transactions = Vec::new();
        if let Some(txs) = json["transactions"].as_array() {
            for tx in txs {
                let signature = tx["transaction"]["signatures"]
                .as_array()
                .and_then(|arr| arr[0].as_str())
                .unwrap_or_default()
                .to_string();
                
                let signers: Vec<String> = tx["transaction"]["message"]["accountKeys"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

                let program = tx["transaction"]["message"]["instructions"]
                .as_array()
                .and_then(|arr| arr[0]["programId"].as_str())
                .unwrap_or_default()
                .to_string();

                let status = if tx["meta"]["err"].is_null() {
                    "success".to_string()
                } else {
                    "failed".to_string()
                };
                
                transactions.push(ParsedTransaction {
                    signature,
                    signers,
                    program,
                    status,
                });
            }
        }

        Some(ParsedBlock {
            slot,
            parent_slot,
            blockhash,
            block_time,
            transactions,
        })

    }

    //high level function fetch,parse and store blocks
    pub async fn process_block(&self, slot:u64)->Result<()>{
        let raw = self.fetch_block(slot).await?;
        if let Some(parsed) = Ledger::parse_block(&raw) {
            self.db.save_block(&parsed)?;
            println!("✅ Stored block {} with {} txs", parsed.slot, parsed.transactions.len());
        }else {
            eprintln!("⚠️ Failed to parse block {}", slot);
        }
        Ok(())
    } 
     

    //fetch all blocks in range
    /**
     * get client 
     * get the body as the payload 
     * 
     */
    pub async fn fetch_slot_in_range(&self, start_slot:u64, end_slot:u64){
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "jsonrpc":"2.0",
            "id": 1,
            "method":"getblocks",
            "params":""
        });
    }
    
}