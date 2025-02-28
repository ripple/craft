use std::collections::HashMap;
use wasmedge_sdk::{
    params, Vm, WasmEdgeResult, AsInstance, Store, Module, Instance, CallingFrame,
    WasmValue, ImportObjectBuilder,
};
use wasmedge_sdk::error::CoreError;
use wasmedge_sdk::vm::SyncInst;
use wasmedge_sdk::wasi::WasiModule;

#[derive(Clone, Debug)]
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

pub fn run_func<T: AsRef<str>>(
    wasm_path: &str,
    func_name: T,
    tx_json: Vec<u8>,
    lo_json: Vec<u8>,
) -> WasmEdgeResult<bool> {
    // Create WASI module
    let mut wasi_module = WasiModule::create(None, None, None)?;

    // Create host functions
    let ledger = LedgerData { sqn: 5 };
    let mut import_builder = ImportObjectBuilder::new("host_lib", ledger)?;
    import_builder
        .with_func::<(), i32>("get_ledger_sqn", get_ledger_sqn)?;
    let mut import_object = import_builder.build();

    // Set up instances
    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    // Create VM and load module
    let mut vm = Vm::new(Store::new(None, instances)?);
    let wasm_module = Module::from_file(None, wasm_path)?;
    vm.register_module(None, wasm_module)?;

    // Allocate memory for transaction JSON
    let tx_size = tx_json.len() as i32;
    let tx_pointer = vm.run_func(None, "allocate", params!(tx_size))?[0].to_i32();
    println!("host tx alloc {} {}", tx_pointer, tx_size);

    // Allocate memory for ledger object JSON
    let lo_size = lo_json.len() as i32;
    let lo_pointer = vm.run_func(None, "allocate", params!(lo_size))?[0].to_i32();
    println!("host lo alloc {} {}", lo_pointer, lo_size);

    // Write data to memory
    let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    memory.set_data(tx_json, tx_pointer as u32)?;
    memory.set_data(lo_json, lo_pointer as u32)?;

    // Call the function
    let rets = vm.run_func(None, func_name, params!(tx_pointer, tx_size, lo_pointer, lo_size))?;
    Ok(rets[0].to_i32() == 1)
}
