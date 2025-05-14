mod vm;
mod mock_data;
mod host_functions;
mod sfield;

use std::collections::HashMap;
use std::path::PathBuf;
use wasmedge_sdk::{wasi::WasiModule, Module, Store, Vm};
use crate::vm::run_func;
use clap::Parser;
use log::{info, debug, error};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::fs;

/// WasmEdge WASM testing utility
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the WASM file
    #[arg(short, long)]
    wasm_file: Option<String>,

    /// Path to the WASM file (alias for backward compatibility)
    #[arg(long)]
    wasm_path: Option<String>,
    
    /// Test case to run (success/failure)
    #[arg(short, long, default_value = "success")]
    test_case: String,
    
    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn load_test_data(test_case: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("escrow")
        .join(test_case);
    
    let tx_path = base_path.join("tx.json");
    let lo_path = base_path.join("ledger_object.json");
    
    let tx_json = fs::read_to_string(tx_path)?;
    let lo_json = fs::read_to_string(lo_path)?;
    
    Ok((tx_json, lo_json))
}

fn main() {
    let args = Args::parse();
    
    // Use wasm_file if provided, otherwise use wasm_path
    let wasm_file = match (&args.wasm_file, &args.wasm_path) {
        (Some(file), _) => file.clone(),
        (None, Some(path)) => path.clone(),
        (None, None) => {
            eprintln!("Error: Either --wasm-file or --wasm-path must be provided");
            std::process::exit(1);
        }
    };
    
    // Initialize logger with appropriate level
    let log_level = if args.verbose { LevelFilter::Debug } else { LevelFilter::Info };
    
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, log_level)
        .init();
    
    info!("Starting WasmEdge host application");
    info!("Loading WASM module from: {}", wasm_file);
    info!("Target function: finish (XLS-100d)");
    info!("Using test case: {}", args.test_case);
    
    debug!("Initializing WasiModule");
    let mut wasi_module = match WasiModule::create(None, None, None) {
        Ok(module) => {
            debug!("WasiModule initialized successfully");
            module
        },
        Err(e) => {
            error!("Failed to create WasiModule: {}", e);
            return;
        }
    };
    
    debug!("Setting up instance map");
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    
    info!("Creating new Vm instance");
    let store = match Store::new(None, instances) {
        Ok(store) => store,
        Err(e) => {
            error!("Failed to create Store: {}", e);
            return;
        }
    };
    
    let mut vm = Vm::new(store);

    info!("Loading WASM module from file: {}", wasm_file);
    let wasm_module = match Module::from_file(None, &wasm_file) {
        Ok(module) => {
            debug!("WASM module loaded successfully");
            module
        },
        Err(e) => {
            error!("Failed to load WASM module from {}: {}", wasm_file, e);
            return;
        }
    };
    
    info!("Registering WASM module to VM");
    if let Err(e) = vm.register_module(None, wasm_module.clone()) {
        error!("Failed to register module: {}", e);
        return;
    }
    debug!("WASM module registered successfully");

    info!("Loading test data from fixtures");
    let (tx_json, lo_json) = match load_test_data(&args.test_case) {
        Ok((tx, lo)) => {
            debug!("Test data loaded successfully");
            (tx, lo)
        },
        Err(e) => {
            error!("Failed to load test data: {}", e);
            return;
        }
    };

    info!("Executing function: finish");
    match run_func(&mut vm, "finish", tx_json.as_bytes().to_vec(), lo_json.as_bytes().to_vec()) {
        Ok(result) => {
            println!("\n-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION RESULT                |");
            println!("-------------------------------------------------");
            println!("| Function:   {:<33} |", "finish");
            println!("| Test Case:  {:<33} |", args.test_case);
            println!("| Result:     {:<33} |", result);
            println!("-------------------------------------------------");
            info!("Function completed successfully with result: {}", result);
        },
        Err(e) => {
            println!("\n-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION ERROR                 |");
            println!("-------------------------------------------------");
            println!("| Function:   {:<33} |", "finish");
            println!("| Test Case:  {:<33} |", args.test_case);
            println!("| Error:      {:<33} |", e);
            println!("-------------------------------------------------");
            error!("Function execution failed: {}", e);
        }
    }
    
    info!("WasmEdge host application execution completed");
}

