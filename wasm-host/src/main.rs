mod call_recorder;
mod data_provider;
mod decoding;
mod hashing;
mod host_function_utils;
mod host_functions;
mod mock_data;
mod recording_host_functions;
mod sfield;
mod vm;

use crate::call_recorder::{CallRecorder, HostCall};
use crate::mock_data::MockData;
use crate::vm::{run_func, run_func_with_recording};
use std::cell::RefCell;
use std::rc::Rc;
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

    /// Host function test mode (uses new fixture system)
    #[arg(long)]
    host_function_test: Option<String>,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn load_test_data(
    test_case: &str,
) -> Result<(String, String, String, String, String), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("escrow")
        .join(test_case);

    let tx_path = base_path.join("tx.json");
    let lo_path = base_path.join("ledger_object.json");
    let lh_path = base_path.join("ledger_header.json");
    let l_path = base_path.join("ledger.json");
    let nfts_path = base_path.join("nfts.json");

    let tx_json = fs::read_to_string(tx_path)?;
    let lo_json = fs::read_to_string(lo_path)?;
    let lh_json = fs::read_to_string(lh_path)?;
    let l_json = fs::read_to_string(l_path)?;
    let nft_json = fs::read_to_string(nfts_path)?;

    Ok((tx_json, lo_json, lh_json, l_json, nft_json))
}

fn load_host_function_test(
    test_path: &str,
) -> Result<(serde_json::Value, Vec<HostCall>), Box<dyn std::error::Error>> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("host_functions")
        .join(test_path);

    let config_path = base_path.join("config.json");
    let input_path = base_path.join("input.json");
    let expected_path = base_path.join("expected.json");

    let config_json = fs::read_to_string(config_path)?;
    let input_json = fs::read_to_string(input_path)?;
    let expected_json = fs::read_to_string(expected_path)?;

    let _config: serde_json::Value = serde_json::from_str(&config_json)?;
    let input: serde_json::Value = serde_json::from_str(&input_json)?;
    let expected: serde_json::Value = serde_json::from_str(&expected_json)?;

    let expected_calls: Vec<HostCall> = serde_json::from_value(
        expected["expected_host_calls"].clone()
    )?;

    Ok((input, expected_calls))
}

fn verify_host_calls(
    actual: &[HostCall],
    expected: &[HostCall],
) -> Result<(), String> {
    if actual.len() != expected.len() {
        return Err(format!(
            "Call count mismatch: expected {}, got {}",
            expected.len(),
            actual.len()
        ));
    }

    for (i, (actual_call, expected_call)) in actual.iter().zip(expected.iter()).enumerate() {
        if actual_call.function != expected_call.function {
            return Err(format!(
                "Call {} function mismatch: expected '{}', got '{}'",
                i + 1, expected_call.function, actual_call.function
            ));
        }

        if actual_call.call_order != expected_call.call_order {
            return Err(format!(
                "Call {} order mismatch: expected {}, got {}",
                i + 1, expected_call.call_order, actual_call.call_order
            ));
        }

        // Verify specific parameter types match
        match (&actual_call.parameters, &expected_call.parameters) {
            (
                crate::call_recorder::HostCallParams::UpdateData { data: actual_data, .. },
                crate::call_recorder::HostCallParams::UpdateData { data: expected_data, .. }
            ) => {
                if actual_data != expected_data {
                    return Err(format!(
                        "Call {} update_data mismatch: expected {:?}, got {:?}",
                        i + 1, expected_data, actual_data
                    ));
                }
            }
            (
                crate::call_recorder::HostCallParams::Trace { message: actual_msg, data: actual_data, .. },
                crate::call_recorder::HostCallParams::Trace { message: expected_msg, data: expected_data, .. }
            ) => {
                if actual_msg != expected_msg {
                    return Err(format!(
                        "Call {} trace message mismatch: expected '{}', got '{}'",
                        i + 1, expected_msg, actual_msg
                    ));
                }
                if actual_data != expected_data {
                    return Err(format!(
                        "Call {} trace data mismatch: expected {:?}, got {:?}",
                        i + 1, expected_data, actual_data
                    ));
                }
            }
            _ => {
                // For now, skip detailed parameter verification for other types
            }
        }
    }

    Ok(())
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

    // Check if we're in host function test mode
    if let Some(host_function_test) = &args.host_function_test {
        info!("Running host function test: {}", host_function_test);
        
        // Load host function test data
        let (_input_data, expected_calls) = match load_host_function_test(host_function_test) {
            Ok((input, expected)) => {
                debug!("Host function test data loaded successfully");
                (input, expected)
            }
            Err(e) => {
                error!("Failed to load host function test data: {}", e);
                return;
            }
        };

        // Create call recorder
        let recorder = Rc::new(RefCell::new(CallRecorder::new()));

        // Use minimal mock data for host function tests (we're not testing escrow functionality)
        let minimal_tx = r#"{"TransactionType": "EscrowFinish", "Account": "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH"}"#.to_string();
        let minimal_lo = r#"{"LedgerEntryType": "Escrow", "Account": "rN7n7otQDd6FczFgLdSqtcsAUxDkw6fzRH"}"#.to_string();
        let minimal_lh = r#"{"ledger_index": 123}"#.to_string();
        let minimal_l = r#"[]"#.to_string();
        let minimal_nft = r#"[]"#.to_string();

        let data_source = MockData::new(&minimal_tx, &minimal_lo, &minimal_lh, &minimal_l, &minimal_nft);
        
        info!("Executing function with call recording: finish");
        match run_func_with_recording(wasm_file, "finish", data_source, recorder.clone()) {
            Ok(result) => {
                info!("Function completed with result: {}", result);
                
                // Verify the calls
                let actual_calls: Vec<HostCall> = recorder.borrow().get_calls().iter().cloned().collect();
                match verify_host_calls(&actual_calls, &expected_calls) {
                    Ok(()) => {
                        println!("\n-------------------------------------------------");
                        println!("| HOST FUNCTION TEST PASSED                    |");
                        println!("-------------------------------------------------");
                        println!("| Test:       {:<33} |", host_function_test);
                        println!("| Calls:      {:<33} |", actual_calls.len());
                        println!("| Result:     {:<33} |", result);
                        println!("-------------------------------------------------");
                        info!("Host function test PASSED");
                    }
                    Err(e) => {
                        println!("\n-------------------------------------------------");
                        println!("| HOST FUNCTION TEST FAILED                    |");
                        println!("-------------------------------------------------");
                        println!("| Test:       {:<33} |", host_function_test);
                        println!("| Error:      {:<33} |", e);
                        println!("-------------------------------------------------");
                        
                        // Print detailed call comparison
                        println!("\nACTUAL CALLS:");
                        for (i, call) in actual_calls.iter().enumerate() {
                            println!("{}. {}: {:?}", i + 1, call.function, call.parameters);
                        }
                        
                        println!("\nEXPECTED CALLS:");
                        for (i, call) in expected_calls.iter().enumerate() {
                            println!("{}. {}: {:?}", i + 1, call.function, call.parameters);
                        }
                        
                        error!("Host function test FAILED: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                println!("\n-------------------------------------------------");
                println!("| WASM FUNCTION EXECUTION ERROR                 |");
                println!("-------------------------------------------------");
                println!("| Function:   {:<33} |", "finish");
                println!("| Test:       {:<33} |", host_function_test);
                println!("| Error:      {:<33} |", e);
                println!("-------------------------------------------------");
                error!("Function execution failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Original escrow test mode
        info!("Target function: finish (XLS-100d)");
        info!("Using test case: {}", args.test_case);
        info!("Loading test data from fixtures");
        let (tx_json, lo_json, lh_json, l_json, nft_json) = match load_test_data(&args.test_case) {
            Ok((tx, lo, lh, l, nft)) => {
                debug!("Test data loaded successfully");
                (tx, lo, lh, l, nft)
            }
            Err(e) => {
                error!("Failed to load test data: {}", e);
                return;
            }
        };

        let data_source = MockData::new(&tx_json, &lo_json, &lh_json, &l_json, &nft_json);
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
    }

    info!("WasmEdge host application execution completed");
}
