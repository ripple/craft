mod data_provider;
mod decoding;
mod hashing;
mod host_functions;
mod mock_data;
mod sfield;
mod vm;

use crate::mock_data::MockData;
use crate::vm::run_func;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, error, info};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

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

fn load_test_data(
    test_case: &str,
) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("escrow")
        .join(test_case);

    let tx_path = base_path.join("tx.json");
    let lo_path = base_path.join("ledger_object.json");
    let lh_path = base_path.join("ledger_header.json");
    let l_path = base_path.join("ledger.json");

    let tx_json = fs::read_to_string(tx_path)?;
    let lo_json = fs::read_to_string(lo_path)?;
    let lh_json = fs::read_to_string(lh_path)?;
    let l_json = fs::read_to_string(l_path)?;

    Ok((tx_json, lo_json, lh_json, l_json))
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
    let log_level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

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

    info!("Starting WasmEdge host application {:?}", args);
    info!("Loading WASM module from: {}", wasm_file);
    info!("Target function: finish (XLS-100d)");
    info!("Using test case: {}", args.test_case);
    info!("Loading test data from fixtures");
    let (tx_json, lo_json, lh_json, l_json) = match load_test_data(&args.test_case) {
        Ok((tx, lo, lh, l)) => {
            debug!("Test data loaded successfully");
            (tx, lo, lh, l)
        }
        Err(e) => {
            error!("Failed to load test data: {}", e);
            return;
        }
    };

    let data_source = MockData::new(&tx_json, &lo_json, &lh_json, &l_json);
    info!("Executing function: finish");
    match run_func(wasm_file, "finish", data_source) {
        Ok(result) => {
            println!("\n-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION RESULT                |");
            println!("-------------------------------------------------");
            println!("| Function:   {:<33} |", "finish");
            println!("| Test Case:  {:<33} |", args.test_case);
            println!("| Result:     {:<33} |", result);
            println!("-------------------------------------------------");
            info!("Function completed successfully with result: {}", result);
        }
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
