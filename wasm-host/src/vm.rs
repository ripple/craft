use log::{error, info};
use wasmedge_sdk::{Vm, params, vm::SyncInst};

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
    // tx_data: Vec<u8>,
    // lo_data: Vec<u8>
) -> Result<bool, Box<dyn std::error::Error>> {
    info!("Executing WASM function: {}", func_name);
    // debug!(
    //     "TX data size: {} bytes, LO data size: {} bytes",
    //     tx_data.len(),
    //     lo_data.len()
    // );

    // Parse and log JSON data for debugging
    // if log::log_enabled!(log::Level::Debug) {
    //     if let Ok(tx_json) = std::str::from_utf8(&tx_data) {
    //         debug!("TX JSON: {}", tx_json);
    //     }
    //     if let Ok(lo_json) = std::str::from_utf8(&lo_data) {
    //         debug!("LO JSON: {}", lo_json);
    //     }
    // }

    // let tx_size = 100; // tx_data.len() as i32;
    // let lo_size = 99; // lo_data.len() as i32;

    let rets = match vm.run_func(
        None,
        func_name,
        params!(
            // tx_pointer, tx_size, lo_pointer, lo_size
        ),
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

// TODO:
// WRITE_WASM_MEMORY_AND_RETURN(
// write_ptr,
// txID.size(),
// txID.data(),
// txID.size(),
// memory,
// memory_length);
pub fn write_wasm_memory_and_return() {

    // TODO: Calls write_wasm_memory
}

pub fn write_wasm_memory(// bytes_written,                                                          \
    // guest_dst_ptr,                                                          \
    // guest_dst_len,                                                          \
    // host_src_ptr,                                                           \
    // host_src_len,                                                           \
    // host_memory_ptr,                                                        \
    // guest_memory_length)                                                    \
) {
}
