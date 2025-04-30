use std::collections::HashMap;
use log::{debug, error, info};
use wasmedge_sdk::{Vm, params, vm::SyncInst, ImportObjectBuilder, AsInstance, Store, Module};
use wasmedge_sdk::error::WasmEdgeError;
use crate::mock_data::MockLedgerTransactionData;

/// Run a WASM function 
pub fn run_func(wasm_file: String, func_name: &str, data_provider: MockLedgerTransactionData)
                -> Result<bool, Box<WasmEdgeError>> {

    let mut import_builder = ImportObjectBuilder::new("host_lib", data_provider).unwrap();
    import_builder.with_func::<i32, ()>("getLedgerSqn", get_ledger_sqn)?;
    import_builder.with_func::<(), i32>("getParentLedgerTime", get_parent_ledger_time)?;
    let mut import_object = import_builder.build();

    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    // instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    info!("Creating new Vm instance");
    let store = match Store::new(None, instances) {
        Ok(store) => store,
        Err(e) => {
            error!("Failed to create Store: {}", e);
            return Err(e);
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
            return Err(e);
        }
    };

    info!("Registering WASM module to VM");
    if let Err(e) = vm.register_module(None, wasm_module.clone()) {
        error!("Failed to register module: {}", e);
        return Err(e);
    }
    debug!("WASM module registered successfully");
    
    
    
    info!("Executing WASM function: {}", func_name);
    let rets = match vm.run_func(
        None,
        func_name,
        params!(),
    ) {
        Ok(values) => values,
        Err(e) => {
            error!("Function execution failed: {}", e);
            return Err(e);
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
