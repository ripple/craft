use log::{error, info};
use std::error::Error;
use wasmedge_sdk::{Vm, params, vm::SyncInst};

/// Run a WASM function with two JSON data parameters
///
/// This function is designed to handle WASM smart contract functions that take:
/// - A transaction JSON (tx_data)
/// - A ledger object JSON (lo_data)
///
/// The function expects the WASM module to expose an "allocate" function that allocates memory
/// for the host to write data into.
pub fn run_func<T: SyncInst>(vm: &mut Vm<T>, func_name: &str) -> Result<bool, Box<dyn Error>> {
    info!("Executing WASM function: {}", func_name);

    let rets = match vm.run_func(None, func_name, params!()) {
        Ok(values) => values,
        Err(e) => {
            error!("Function execution failed: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("WASM function execution error: {}", e),
            )));
        }
    };

    // Convert the result to a boolean (1 = true, 0 = false)
    let result = rets[0].to_i32() == 1;
    info!("Function '{}' returned: {}", func_name, result);

    Ok(result)
}
