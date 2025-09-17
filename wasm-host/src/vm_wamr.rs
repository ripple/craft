use crate::data_provider::DataProvider;
use crate::host_functions_wamr::{
    account_keylet, amm_keylet, cache_ledger_obj, check_keylet, compute_sha512_half,
    credential_keylet, delegate_keylet, deposit_preauth_keylet, did_keylet, escrow_keylet,
    float_add, float_compare, float_divide, float_from_int, float_from_uint, float_log,
    float_multiply, float_pow, float_root, float_set, float_subtract,
    get_current_ledger_obj_array_len, get_current_ledger_obj_field,
    get_current_ledger_obj_nested_array_len, get_current_ledger_obj_nested_field,
    get_ledger_obj_array_len, get_ledger_obj_field, get_ledger_obj_nested_array_len,
    get_ledger_obj_nested_field, get_ledger_sqn, get_nft, get_parent_ledger_hash,
    get_parent_ledger_time, get_tx_array_len, get_tx_field, get_tx_nested_array_len,
    get_tx_nested_field, line_keylet, mpt_issuance_keylet, mptoken_keylet, nft_offer_keylet,
    offer_keylet, oracle_keylet, paychan_keylet, permissioned_domain_keylet, signers_keylet,
    ticket_keylet, trace, trace_account, trace_amount, trace_num, trace_opaque_float, update_data,
    vault_keylet,
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
use std::time::Instant;

#[rustfmt::skip]
#[allow(unused)]
pub fn run_func(wasm_file: String, func_name: &str, gas_cap: Option<u32>, data_source: MockData) -> Result<bool, RuntimeError>{
    debug!("Setting up wamr runtime and registering host functions");
    let mut data_provider = DataProvider::new(data_source);
    let t0 = Instant::now();
    let mut pool = vec![0u8; 1024 * 1024];
    let runtime = Runtime::builder().run_as_interpreter().use_memory_pool(pool)
        .use_system_allocator()
        .register_host_function("get_ledger_sqn", get_ledger_sqn as *mut c_void, "()i", 60, data_provider.as_ptr())
        .register_host_function("get_parent_ledger_time", get_parent_ledger_time as *mut c_void, "()i", 60, data_provider.as_ptr())
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
        .register_host_function("amm_keylet", amm_keylet as *mut c_void, "(*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("credential_keylet", credential_keylet as *mut c_void, "(*~*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("check_keylet", check_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("delegate_keylet", delegate_keylet as *mut c_void, "(*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("deposit_preauth_keylet", deposit_preauth_keylet as *mut c_void, "(*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("did_keylet", did_keylet as *mut c_void, "(*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("escrow_keylet", escrow_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("line_keylet", line_keylet as *mut c_void, "(*~*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("mpt_issuance_keylet", mpt_issuance_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("mptoken_keylet", mptoken_keylet as *mut c_void, "(*~*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("nft_offer_keylet", nft_offer_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("offer_keylet", offer_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("oracle_keylet", oracle_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("paychan_keylet", paychan_keylet as *mut c_void, "(*~*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("permissioned_domain_keylet", permissioned_domain_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("signers_keylet", signers_keylet as *mut c_void, "(*~*~)i", 350, data_provider.as_ptr())
        .register_host_function("ticket_keylet", ticket_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("vault_keylet", vault_keylet as *mut c_void, "(*~i*~)i", 350, data_provider.as_ptr())
        .register_host_function("get_nft", get_nft as *mut c_void, "(*~*~*~)i", 1000, data_provider.as_ptr())
        .register_host_function("float_from_int", float_from_int as *mut c_void, "(I*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_from_uint", float_from_uint as *mut c_void, "(*~*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_set", float_set as *mut c_void, "(iI*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_compare", float_compare as *mut c_void, "(*~*~)i", 1000, data_provider.as_ptr())
        .register_host_function("float_add", float_add as *mut c_void, "(*~*~*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_subtract", float_subtract as *mut c_void, "(*~*~*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_multiply", float_multiply as *mut c_void, "(*~*~*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_divide", float_divide as *mut c_void, "(*~*~*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_pow", float_pow as *mut c_void, "(*~i*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_root", float_root as *mut c_void, "(*~i*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("float_log", float_log as *mut c_void, "(*~*~i)i", 1000, data_provider.as_ptr())
        .register_host_function("trace", trace as *mut c_void, "(*~*~i)i", 500, data_provider.as_ptr())
        .register_host_function("trace_num", trace_num as *mut c_void, "(*~I)i", 500, data_provider.as_ptr())
        .register_host_function("trace_opaque_float", trace_opaque_float as *mut c_void, "(*~*~)i", 500, data_provider.as_ptr())
        .register_host_function("trace_account", trace_account as *mut c_void, "(*~*~)i", 500, data_provider.as_ptr())
        .register_host_function("trace_amount", trace_amount as *mut c_void, "(*~*~)i", 500, data_provider.as_ptr())
        .build()?;

    debug!("Loading WASM module from file: {}", wasm_file);
    let wasm_path = PathBuf::from(wasm_file);
    let module = Module::from_file(&runtime, wasm_path.as_path())?;
    // rippled currently allows 128kb for each VM instance, so we use the same here in Craft.
    let instance = Instance::new(&runtime, &module, 1024 * 128)?;

    debug!("Executing WASM function: {}", func_name);
    let func = Function::find_export_func(&instance, "finish")?;
    let gas_begin = gas_cap.map_or(0, |x|x);
    let t1 = Instant::now();
    let results = func.call(&instance, &vec![], gas_cap)?;
    let elapsed0 = t0.elapsed();
    let elapsed1 = t1.elapsed();
    info!("whole execution time: {} ms", elapsed0.as_secs_f64() * 1000.0);
    info!("func execution time : {} ms", elapsed1.as_secs_f64() * 1000.0);
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
