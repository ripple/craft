use crate::host_functions::{
    account_keylet, compute_sha512_half, credential_keylet, escrow_keylet,
    get_current_ledger_obj_array_len, get_current_ledger_obj_field,
    get_current_ledger_obj_nested_array_len, get_current_ledger_obj_nested_field,
    get_ledger_obj_array_len, get_ledger_obj_field, get_ledger_obj_nested_array_len,
    get_ledger_obj_nested_field, get_ledger_sqn, get_parent_ledger_hash,
    get_parent_ledger_time, get_tx_array_len, get_tx_field, get_tx_nested_array_len,
    get_tx_nested_field, ledger_slot_set, oracle_keylet, trace, trace_num, update_data};

use crate::mock_data::MockData;
use log::{debug, info};
use std::collections::HashMap;
use wasmedge_sdk::{params, AsInstance, ImportObjectBuilder, Module, Store, Vm, WasmEdgeResult};
use wasmedge_sdk::vm::SyncInst;
use crate::data_provider::DataProvider;

/// Run a WASM function
pub fn run_func(wasm_file: String, func_name: &str, data_source: MockData) -> WasmEdgeResult<bool> {
    info!("Executing WASM function: {}", func_name);
    let data_provider = DataProvider::new(data_source);

    debug!("Setting up instance map and registering host functions");
    let mut instances : HashMap<String, &mut dyn SyncInst> = HashMap::new();
    let mut import_builder = ImportObjectBuilder::new("host_lib", data_provider)?;

    info!("Linking `trace` function");
    import_builder.with_func::<(i32, i32, i32, i32, i32), i32>("trace", trace)?;
    info!("Linking `trace_num` function");
    import_builder.with_func::<(i32, i32, i64), i64>("trace_num", trace_num)?;

    import_builder.with_func::<(i32, i32), i32>("get_ledger_sqn", get_ledger_sqn)?;
    import_builder.with_func::<(i32, i32), i32>("get_parent_ledger_time", get_parent_ledger_time)?;
    import_builder.with_func::<(i32, i32), i32>("get_parent_ledger_hash", get_parent_ledger_hash)?;
    import_builder.with_func::<(i32, i32, i32), i32>("ledger_slot_set", ledger_slot_set)?;
    import_builder.with_func::<(i32, i32, i32), i32>("get_tx_field", get_tx_field)?;
    import_builder.with_func::<(i32, i32, i32), i32>("get_current_ledger_obj_field", get_current_ledger_obj_field)?;
    import_builder.with_func::<(i32, i32, i32, i32), i32>("get_ledger_obj_field", get_ledger_obj_field)?;
    import_builder.with_func::<(i32, i32, i32, i32), i32>("get_tx_nested_field", get_tx_nested_field)?;
    import_builder.with_func::<(i32, i32, i32, i32), i32>("get_current_ledger_obj_nested_field", get_current_ledger_obj_nested_field)?;
    import_builder.with_func::<(i32, i32, i32, i32, i32), i32>("get_ledger_obj_nested_field", get_ledger_obj_nested_field)?;
    import_builder.with_func::<i32, i32>("get_tx_array_len", get_tx_array_len)?;
    import_builder.with_func::<i32, i32>("get_current_ledger_obj_array_len", get_current_ledger_obj_array_len)?;
    import_builder.with_func::<(i32, i32), i32>("get_ledger_obj_array_len", get_ledger_obj_array_len)?;
    import_builder.with_func::<(i32, i32), i32>("get_tx_nested_array_len", get_tx_nested_array_len)?;
    import_builder.with_func::<(i32, i32), i32>("get_current_ledger_obj_nested_array_len", get_current_ledger_obj_nested_array_len)?;
    import_builder.with_func::<(i32, i32, i32), i32>("get_ledger_obj_nested_array_len", get_ledger_obj_nested_array_len)?;
    import_builder.with_func::<(i32, i32), ()>("update_data", update_data)?;
    import_builder.with_func::<(i32, i32, i32, i32), i32>("compute_sha512_half", compute_sha512_half)?;
    import_builder.with_func::<(i32, i32, i32, i32), i32>("account_keylet", account_keylet)?;
    import_builder.with_func::<(i32, i32, i32, i32, i32, i32, i32, i32), i32>("credential_keylet", credential_keylet)?;
    import_builder.with_func::<(i32, i32, i32, i32, i32), i32>("escrow_keylet", escrow_keylet)?;
    import_builder.with_func::<(i32, i32, i32, i32, i32), i32>("oracle_keylet", oracle_keylet)?;
    //import_builder.with_func::<(i32, i32, i32, i32), i32>("", )?;
    let mut import_object = import_builder.build();
    instances.insert(import_object.name().unwrap(), &mut import_object);
    // keep wasi commented out, but keep here for println!
    // let mut wasi_module = wasmedge_sdk::wasi::WasiModule::create(None, None, None)?;
    // instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    info!("Creating new Vm instance");
    let mut vm = Vm::new(Store::new(None, instances)?);

    info!("Loading WASM module from file: {}", wasm_file);
    let wasm_module = Module::from_file(None, &wasm_file)?;

    info!("Registering WASM module to VM");
    vm.register_module(None, wasm_module.clone())?;

    let rets = vm.run_func(None, func_name, params!())?;
    // println!("run_func: {:?}", rets[0].to_i32());
    Ok(rets[0].to_i32() == 1)
}
