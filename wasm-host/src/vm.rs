use wasmedge_sdk::{
    params, Vm, Instance, CallingFrame, AsInstance,
    WasmValue, WasmVal, vm::SyncInst
};
use wasmedge_sdk::error::CoreError;

struct LedgerData {
    sqn: i32,
}

fn get_ledger_sqn(
    data: &mut LedgerData,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    Ok(vec![WasmValue::from_i32(data.sqn)])
}

/// Run a WASM function with two JSON data parameters
/// 
/// This function is designed to handle WASM smart contract functions that take:
/// - A transaction JSON (tx_data)
/// - A ledger object JSON (lo_data)
///
/// The function expects the WASM module to expose an "allocate" function that allocates memory
/// for the host to write data into.
pub fn run_func<T: SyncInst>(vm: &mut Vm<T>, func_name: &str, tx_data: Vec<u8>, lo_data: Vec<u8>) -> Result<bool, Box<dyn std::error::Error>> {
    let tx_size = tx_data.len() as i32;
    let lo_size = lo_data.len() as i32;

    // Allocate memory for transaction data
    let tx_pointer = vm.run_func(None, "allocate", params!(tx_size))?[0].to_i32();
    
    // Allocate memory for ledger object data
    let lo_pointer = vm.run_func(None, "allocate", params!(lo_size))?[0].to_i32();

    // Get mutable access to the memory
    let active_module = vm.active_module_mut()
        .ok_or("Failed to get active module")?;
    
    let mut memory = active_module.get_memory_mut("memory")?;
    
    // Write data to memory
    memory.set_data(tx_data, tx_pointer as u32)?;
    memory.set_data(lo_data, lo_pointer as u32)?;

    // Call the target function with pointers and sizes
    let rets = vm.run_func(None, func_name, params!(tx_pointer, tx_size, lo_pointer, lo_size))?;
    
    // Convert the result to a boolean (1 = true, 0 = false)
    Ok(rets[0].to_i32() == 1)
}
