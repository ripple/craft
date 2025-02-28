mod vm;

use std::collections::HashMap;
use wasmedge_sdk::{wasi::WasiModule, Module, Store, Vm};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the wasm file
    #[arg(short, long)]
    wasm_file: String,

    /// Function to test
    #[arg(short, long)]
    function: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Test transaction JSON
    let tx_json = r#"{
        "escrow": {
            "accountId": "alice.near",
            "amount": "100",
            "receiver": "bob.near",
            "lockHeight": 1000
        }
    }"#.as_bytes().to_vec();

    // Test ledger object JSON
    let lo_json = r#"{
        "accountId": "alice.near",
        "balance": "1000",
        "nonce": 5
    }"#.as_bytes().to_vec();

    // Run the function
    let result = vm::run_func(
        &cli.wasm_file,
        &cli.function,
        tx_json,
        lo_json,
    ).map_err(|e| anyhow::anyhow!("WasmEdge error: {}", e))?;

    println!("Function result: {}", result);
    Ok(())
}

//r#"{"Account":"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh","TransactionType":"EscrowFinish","Flags":0,"Owner":"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh","OfferSequence":7,"Condition":"DEADBEEF","Fulfillment":"DEADBEEF"}"#;

// let escrow_lo_json_str: Vec<u8> = escrow_json_str.as_bytes().to_vec();
// // println!("vec u8 from str {:?}", escrow_tx_json_bytes);
// let escrow_lo_json_bytes = escrow_tx_json_bytes.clone();

