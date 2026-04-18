use anyhow::Result;
use serde_json::Value;

use crate::ledger::{self, Ledger};



struct PoH{
    pub start_hash : String,
    pub current_hash : String,
    pub ledger : Ledger,
    pub verified : Vec<String> , //the chain of hashes we have 
}


impl PoH {
    pub fn new(start_hash :String, current_hash:String,ledger:Ledger)->Self{
        Self { start_hash, current_hash, ledger, verified:Vec::new() }
    }
    
    pub async fn get_block_fetcher(&self, slot :u64)->Result<Value>{
         self.ledger.fetch_block( slot).await
    }
    
    //fetching a verified slot from ranging from a point to another point
    //pub async fn verify_slot_range(&self, start_slot:u64, end_slot:u64){
      //  self.ledger.fetch_block(slot)
    //}

    //the pulls the poh hashes when transaction has been inserted 
    //pub fn poh_entry_extractor(&self)->Result<()>{

    //}
}




