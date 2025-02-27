use wasmedge_sdk::{params, Vm, WasmVal, WasmEdgeResult, AsInstance};
use anyhow::{Context, Result};

pub fn run_string_func<T: wasmedge_sdk::vm::SyncInst>(
    vm: &mut Vm<T>,
    func_name: impl AsRef<str>,
) -> Result<String> {
    let result = vm.run_func(None, func_name, vec![])
        .context("Failed to run function")?;
    
    // Get the pointer and length of the returned string
    let ptr = result[0].to_i32() as u32;
    let len = result[1].to_i32() as u32;
    
    // Read the string from memory
    let memory = vm.active_module_mut()
        .context("No active module")?
        .get_memory("memory")
        .context("No memory found")?;
    
    let mut buffer = vec![0u8; len as usize];
    memory.get_data(&mut buffer, ptr)
        .context("Failed to read memory")?;
    
    String::from_utf8(buffer)
        .context("Failed to convert bytes to string")
}

pub fn run_set_greeting<T: wasmedge_sdk::vm::SyncInst>(
    vm: &mut Vm<T>,
    greeting: &str,
) -> Result<()> {
    // Allocate memory for the string
    let size = greeting.len() as i32;
    let ptr = vm.run_func(None, "allocate", params!(size))
        .context("Failed to allocate memory")?[0].to_i32();
    
    // Write the string to memory
    let memory = vm.active_module_mut()
        .context("No active module")?
        .get_memory_mut("memory")
        .context("No memory found")?;
    
    memory.set_data(greeting.as_bytes().to_vec(), ptr as u32)
        .context("Failed to write to memory")?;
    
    // Call set_greeting with the pointer and length
    vm.run_func(None, "set_greeting", params!(ptr, size))
        .context("Failed to set greeting")?;
    
    Ok(())
}

pub fn run_func<T: wasmedge_sdk::vm::SyncInst>(
    vm : &mut Vm<T>,
    func_name: impl AsRef<str>,
    tx_json: Vec<u8>,
    lo_json: Vec<u8>,
) -> WasmEdgeResult<bool> {

    let tx_size = tx_json.len() as i32;
    let tx_pointer = match vm.run_func(None, "allocate", params!(tx_size)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    println!("host tx alloc {} {}", tx_pointer, tx_size);

    let lo_size = lo_json.len() as i32;
    let lo_pointer = match vm.run_func(None, "allocate", params!(lo_size)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    println!("host lo alloc {} {}", lo_pointer, lo_size);

    let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    memory.set_data(tx_json, tx_pointer as u32).unwrap();
    memory.set_data(lo_json, lo_pointer as u32).unwrap();

    let rets = vm.run_func(None, func_name, params!(tx_pointer, tx_size, lo_pointer, lo_size))?;
    Ok(rets[0].to_i32() == 1)
}
