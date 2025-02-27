mod vm;

use std::collections::HashMap;
use wasmedge_sdk::{wasi::WasiModule, Module, Store, Vm};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(author, version, about = "Test runner for WASM smart contracts")]
struct Cli {
    /// Path to the WASM file to test
    #[arg(short, long)]
    wasm_path: String,

    /// Function to test
    #[arg(short, long, default_value = "get_greeting")]
    function: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize WasmEdge VM
    let mut wasi_module = WasiModule::create(None, None, None)
        .context("Failed to create WASI module")?;
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    let mut vm = Vm::new(Store::new(None, instances)
        .context("Failed to create store")?);

    // Load the WASM module
    println!("Loading WASM module from: {}", cli.wasm_path);
    let wasm_module = Module::from_file(None, &cli.wasm_path)
        .context("Failed to load WASM module")?;
    vm.register_module(None, wasm_module.clone())
        .context("Failed to register module")?;

    // Create a new contract instance
    println!("Creating new contract instance...");
    vm.run_func(None, "new", vec![])
        .context("Failed to create contract instance")?;

    // Test the specified function
    println!("Testing function: {}", cli.function);
    match cli.function.as_str() {
        "get_greeting" => {
            let result = vm::run_string_func(&mut vm, "get_greeting")?;
            println!("Greeting: {}", result);
        }
        "reset_to_default" => {
            vm.run_func(None, "reset_to_default", vec![])
                .context("Failed to reset greeting")?;
            let result = vm::run_string_func(&mut vm, "get_greeting")?;
            println!("Reset greeting to: {}", result);
        }
        "set_greeting" => {
            let new_greeting = "Hello from WasmEdge!";
            vm::run_set_greeting(&mut vm, new_greeting)?;
            let result = vm::run_string_func(&mut vm, "get_greeting")?;
            println!("Set new greeting: {}", result);
        }
        _ => {
            println!("Unknown function: {}", cli.function);
            println!("Available functions:");
            println!("  - get_greeting");
            println!("  - set_greeting");
            println!("  - reset_to_default");
        }
    }

    Ok(())
}

//r#"{"Account":"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh","TransactionType":"EscrowFinish","Flags":0,"Owner":"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh","OfferSequence":7,"Condition":"DEADBEEF","Fulfillment":"DEADBEEF"}"#;

// let escrow_lo_json_str: Vec<u8> = escrow_json_str.as_bytes().to_vec();
// // println!("vec u8 from str {:?}", escrow_tx_json_bytes);
// let escrow_lo_json_bytes = escrow_tx_json_bytes.clone();

