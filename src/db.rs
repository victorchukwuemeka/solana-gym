use anyhow::{Ok, Result};
use rusqlite::{params, Connection, Transaction};
use tracing_appender::non_blocking::NonBlockingBuilder;

use crate::ledger::ParsedTransaction;
use crate::ledger::ParsedBlock;

pub struct Database{
    pub path : String,
}


impl Database {

    pub fn new(path : String)-> Self{
        Self { path }
    }
    
    pub fn init_db(&self)->Result<Connection>{
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS blocks(
                slot  INTEGER PRIMARY KEY,
                parent_slot INTEGER,
                block_hash TEXT,
                timestamp INTEGER 
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transactions(
                tx_signature TEXT PRIMARY KEY,
                slot INTEGER,
                signers TEXT, 
                program TEXT,
                status TEXT 
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS  accounts(
                pubkey TEXT PRIMARY KEY, 
                balance INTEGER ,
                owner_program TEXT, 
                data BLOB
            )",
            [],
        )?;
        Ok(conn)
    }

    //saved parsedblocks  into the db 
    pub fn save_block(&self, block: &ParsedBlock)->Result<()>{
        let conn = self.init_db()?;

        conn.execute(
            "INSERT OR REPLACE INTO blocks 
            (slot, parent_slot, block_hash, timestamp)
             VALUES (?1, ?2, ?3, ?4)",
            params![block.slot, block.parent_slot, block.blockhash, block.block_time],
        )?;

        for tx in &block.transactions {
            self.save_transaction(&conn, tx, block.slot)?;
        }

        Ok(())
    }

    pub fn save_transaction(&self, conn: &Connection, tx: &ParsedTransaction, slot: u64)->Result<()>{
       conn.execute(
        "INSERT OR REPLACE INTO transactions(tx_signature,slot,signers,program, status)
        VALUE(?1,?2,?3,?4,?5)", 
        params![
            tx.signature,
            slot,
            tx.signers.join(","), // store signer pubkeys as CSV
            tx.program,
            tx.status
        ])?;
       Ok(())
    }

}

