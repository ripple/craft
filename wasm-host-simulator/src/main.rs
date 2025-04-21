mod host_functions;
mod vm;

mod host_utils;
mod types;

use crate::vm::run_func;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, error, info};
use std::collections::HashMap;
use std::io::Write;
use wasmedge_sdk::{AsInstance, ImportObject, ImportObjectBuilder, Module, Store, Vm};

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
        .format(|buf, record| writeln!(buf, "[{} {}] {}", record.level(), record.target(), record.args()))
        .filter(None, log_level)
        .init();

    info!("Starting WasmEdge host application");
    info!("Loading WASM module from: {}", wasm_file);
    info!("Target function: ready (XLS-100d)");
    info!("Using test case: {}", args.test_case);

    info!("Setting up import module");
    let mut import_builder = ImportObjectBuilder::new("host", ()).unwrap();
    // ### Register Host Functions Here!
    // debug!("Linking `add` function");
    // import_builder.with_func::<(i32, i32), i32>("add", my_add).unwrap();
    info!("Linking `log` function");
    import_builder
        .with_func::<(i32, i32), ()>("log", host_functions::log)
        .unwrap();

    info!("Linking `log_ln` function");
    import_builder
        .with_func::<(i32, i32), ()>("log_ln", host_functions::log_ln)
        .unwrap();

    info!("Linking `log_hex` function");
    import_builder
        .with_func::<(i32, i32), ()>("log_hex", host_functions::log_hex)
        .unwrap();

    info!("Linking `escrow_finish::get_tx_hash` function");
    import_builder
        .with_func::<i32, ()>("get_tx_hash", host_functions::get_tx_hash)
        .unwrap();

    let mut import_object = import_builder.build();
    let mut instances = HashMap::new();
    instances.insert(import_object.name().unwrap(), &mut import_object);

    info!("Creating new VM instance");
    // TODO: Config for determinism
    let store: Store<ImportObject<()>> = match Store::new(None, instances) {
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
        }
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
    match run_func(&mut vm, "ready") {
        Ok(result) => {
            println!("\n-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION RESULT                |");
            println!("-------------------------------------------------");
            println!("| Function:   {:<33} |", "ready");
            println!("| Test Case:  {:<33} |", args.test_case);
            println!("| Result:     {:<33} |", result);
            println!("-------------------------------------------------");
            info!("Function completed successfully with result: {}", result);
        }
        Err(e) => {
            println!("\n-------------------------------------------------");
            println!("| WASM FUNCTION EXECUTION ERROR                 |");
            println!("-------------------------------------------------");
            println!("| Function:   {:<33} |", "ready");
            println!("| Test Case:  {:<33} |", args.test_case);
            println!("| Error:      {:<33} |", e);
            println!("-------------------------------------------------");
            error!("Function execution failed: {}", e);
        }
    }

    info!("WasmEdge host application execution completed");
}
