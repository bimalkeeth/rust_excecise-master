#![allow(dead_code)]
#![allow(unused_variables)]

use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_transactions() -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string("./transactions.json") {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    Ok(t)
}

pub fn get_transaction_b() -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string("./transactions.json")
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}
