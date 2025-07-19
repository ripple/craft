use crate::data_provider::DataProvider;
use crate::host_functions_wamr::{
    account_keylet, cache_ledger_obj, compute_sha512_half, credential_keylet, escrow_keylet,
    get_current_ledger_obj_array_len, get_current_ledger_obj_field,
    get_current_ledger_obj_nested_array_len, get_current_ledger_obj_nested_field,
    get_ledger_obj_array_len, get_ledger_obj_field, get_ledger_obj_nested_array_len,
    get_ledger_obj_nested_field, get_ledger_sqn, get_nft, get_parent_ledger_hash,
    get_parent_ledger_time, get_tx_array_len, get_tx_field, get_tx_nested_array_len,
    get_tx_nested_field, oracle_keylet, trace, trace_num, trace_opaque_float, update_data,
};
use crate::mock_data::MockData;
use log::{debug, info, warn};
use std::ffi::c_void;
use std::path::PathBuf;
use wamr_rust_sdk::RuntimeError;
use wamr_rust_sdk::function::Function;
use wamr_rust_sdk::instance::Instance;
use wamr_rust_sdk::module::Module;
use wamr_rust_sdk::runtime::Runtime;
use wamr_rust_sdk::value::WasmValue;

#[rustfmt::skip]
#[allow(unused)]
pub fn run_func(wasm_file: String, func_name: &str, gas_cap: Option<u32>, data_source: MockData) -> Result<bool, RuntimeError>{
    debug!("Setting up wamr runtime and registering host functions");
    let mut data_provider = DataProvider::new(data_source);
    let runtime = Runtime::builder()
        .use_system_allocator()
        .register_host_function("get_ledger_sqn", get_ledger_sqn as *mut c_void, "(*~)i", 60, data_provider.as_ptr())
        .register_host_function("get_parent_ledger_time", get_parent_ledger_time as *mut c_void, "(*~)i", 60, data_provider.as_ptr())
        .register_host_function("get_parent_ledger_hash", get_parent_ledger_hash as *mut c_void, "(*~)i", 60, data_provider.as_ptr())
        .register_host_function("cache_ledger_obj", cache_ledger_obj as *mut c_void, "(*~i)i", 5000, data_provider.as_ptr())
        .register_host_function("get_tx_field", get_tx_field as *mut c_void, "(i*~)i", 70, data_provider.as_ptr())
        .register_host_function("get_current_ledger_obj_field", get_current_ledger_obj_field as *mut c_void, "(i*~)i", 70, data_provider.as_ptr())
        .register_host_function("get_ledger_obj_field", get_ledger_obj_field as *mut c_void, "(ii*~)i", 70, data_provider.as_ptr())
        .register_host_function("get_tx_nested_field", get_tx_nested_field as *mut c_void, "(*~*~)i", 110, data_provider.as_ptr())
        .register_host_function("get_current_ledger_obj_nested_field", get_current_ledger_obj_nested_field as *mut c_void, "(*~*~)i", 110, data_provider.as_ptr())
        .register_host_function("get_ledger_obj_nested_field", get_ledger_obj_nested_field as *mut c_void, "(i*~*~)i", 110, data_provider.as_ptr())
        .register_host_function("get_tx_array_len", get_tx_array_len as *mut c_void, "(i)i", 40, data_provider.as_ptr())
        .register_host_function("get_current_ledger_obj_array_len", get_current_ledger_obj_array_len as *mut c_void, "(i)i", 40, data_provider.as_ptr())
        .register_host_function("get_ledger_obj_array_len", get_ledger_obj_array_len as *mut c_void, "(ii)i", 40, data_provider.as_ptr())
        .register_host_function("get_tx_nested_array_len", get_tx_nested_array_len as *mut c_void, "(*~)i", 70, data_provider.as_ptr())
        .register_host_function("get_current_ledger_obj_nested_array_len", get_current_ledger_obj_nested_array_len as *mut c_void, "(*~)i", 70, data_provider.as_ptr())
        .register_host_function("get_ledger_obj_nested_array_len", get_ledger_obj_nested_array_len as *mut c_void, "(i*~)i", 70, data_provider.as_ptr())
        .register_host_function("update_data", update_data as *mut c_void, "(*~)i", 1000, data_provider.as_ptr())
        .register_host_function("compute_sha512_half", compute_sha512_half as *mut c_void, "(*~*~)i", 2000, data_provider.as_ptr())
        .register_host_function("account_keylet", account_keylet as *mut c_void, "(*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("credential_keylet", credential_keylet as *mut c_void, "(*~*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("escrow_keylet", escrow_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("oracle_keylet", oracle_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("get_nft", get_nft as *mut c_void, "(*~*~*~)i", 1000, data_provider.as_ptr())
        .register_host_function("trace", trace as *mut c_void, "(*~*~i)i", 500, data_provider.as_ptr())
        .register_host_function("trace_num", trace_num as *mut c_void, "(*~I)i", 500, data_provider.as_ptr())
        .register_host_function("trace_opaque_float", trace_opaque_float as *mut c_void, "(*~*~)i", 500, data_provider.as_ptr())
        .build()?;

    debug!("Loading WASM module from file: {}", wasm_file);
    let wasm_path = PathBuf::from(wasm_file);
    let module = Module::from_file(&runtime, wasm_path.as_path())?;
    let instance = Instance::new(&runtime, &module, 1024 * 64)?;

    debug!("Executing WASM function: {}", func_name);
    let func = Function::find_export_func(&instance, "finish")?;
    let gas_begin = gas_cap.map_or(0, |x|x);
    let results = func.call(&instance, &vec![], gas_cap)?;
    match results {
        (rv, gas_end) if rv.len() == 1 => {
            if let WasmValue::I32(r1) = rv[0] {
                info!("run_func result: {}", r1);
                if gas_begin > 0 {
                    info!("run_func gas cost: {}", gas_begin - gas_end);
                }
                Ok(r1 == 1)
            } else {
                warn!("Unexpected run_func result vec: {:?}", rv[0]);
                Ok(false)
            }
        }
        _ => {
            warn!("run_func error: {:?}", results);
            Ok(false)
        }
    }
}
