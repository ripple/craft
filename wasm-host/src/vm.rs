use log::{debug, error, info};
use wasmedge_sdk::error::CoreError;
use wasmedge_sdk::{
    params, vm::SyncInst, AsInstance, CallingFrame, Instance, Vm, WasmVal, WasmValue,
};

/// Run a WASM function with two JSON data parameters
///
/// This function is designed to handle WASM smart contract functions that take:
/// - A transaction JSON (tx_data)
/// - A ledger object JSON (lo_data)
///
/// The function expects the WASM module to expose an "allocate" function that allocates memory
/// for the host to write data into.
pub fn run_func<T: SyncInst>(
    vm: &mut Vm<T>,
    func_name: &str,
    tx_data: Vec<u8>,
    lo_data: Vec<u8>,
) -> Result<bool, Box<dyn std::error::Error>> {
    info!("Executing WASM function: {}", func_name);
    debug!(
        "TX data size: {} bytes, LO data size: {} bytes",
        tx_data.len(),
        lo_data.len()
    );

    // Parse and log JSON data for debugging
    if log::log_enabled!(log::Level::Debug) {
        if let Ok(tx_json) = std::str::from_utf8(&tx_data) {
            debug!("TX JSON: {}", tx_json);
        }
        if let Ok(lo_json) = std::str::from_utf8(&lo_data) {
            debug!("LO JSON: {}", lo_json);
        }
    }

    let tx_size = tx_data.len() as i32;
    let lo_size = lo_data.len() as i32;

    // Allocate memory for transaction data
    info!("Allocating memory for transaction data ({} bytes)", tx_size);
    let tx_pointer = match vm.run_func(None, "allocate", params!(tx_size)) {
        Ok(values) => {
            let ptr = values[0].to_i32();
            debug!("Allocated memory at address: 0x{:x}", ptr);
            ptr
        }
        Err(e) => {
            error!("Failed to allocate memory for transaction data: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Memory allocation error: {}", e),
            )));
        }
    };

    // Allocate memory for ledger object data
    info!(
        "Allocating memory for ledger object data ({} bytes)",
        lo_size
    );
    let lo_pointer = match vm.run_func(None, "allocate", params!(lo_size)) {
        Ok(values) => {
            let ptr = values[0].to_i32();
            debug!("Allocated memory at address: 0x{:x}", ptr);
            ptr
        }
        Err(e) => {
            error!("Failed to allocate memory for ledger object data: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Memory allocation error: {}", e),
            )));
        }
    };

    // Get mutable access to the memory
    info!("Getting active module and memory access");
    let active_module = match vm.active_module_mut() {
        Some(module) => module,
        None => {
            error!("Failed to get active module");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get active module",
            )));
        }
    };

    let mut memory = match active_module.get_memory_mut("memory") {
        Ok(mem) => mem,
        Err(e) => {
            error!("Failed to get memory: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Memory access error: {}", e),
            )));
        }
    };

    // Write data to memory
    info!(
        "Writing transaction data to memory at address 0x{:x}",
        tx_pointer
    );
    if let Err(e) = memory.set_data(tx_data, tx_pointer as u32) {
        error!("Failed to write transaction data to memory: {}", e);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Memory write error: {}", e),
        )));
    }

    info!(
        "Writing ledger object data to memory at address 0x{:x}",
        lo_pointer
    );
    if let Err(e) = memory.set_data(lo_data, lo_pointer as u32) {
        error!("Failed to write ledger object data to memory: {}", e);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Memory write error: {}", e),
        )));
    }

    // Call the target function with pointers and sizes
    info!(
        "Calling WASM function '{}' with parameters: (0x{:x}, {}, 0x{:x}, {})",
        func_name, tx_pointer, tx_size, lo_pointer, lo_size
    );

    let rets = match vm.run_func(
        None,
        func_name,
        params!(tx_pointer, tx_size, lo_pointer, lo_size),
    ) {
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
