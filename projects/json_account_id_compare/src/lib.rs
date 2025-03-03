use std::mem;
use serde_json::Value;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    log("allocate called");
    log(&format!("allocating {} bytes", size));
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    log(&format!("allocation address: {:p}", pointer));
    mem::forget(buffer);
    pointer
}

#[no_mangle]
pub extern fn compare_accountID(tx_json_ptr: *mut u8, tx_json_size: usize, lo_json_ptr: *mut u8, lo_json_size: usize) -> bool {
    log("compare_accountID called");
    log(&format!("tx_json_ptr: {:p}, tx_json_size: {}", tx_json_ptr, tx_json_size));
    log(&format!("lo_json_ptr: {:p}, lo_json_size: {}", lo_json_ptr, lo_json_size));
    
    let tx_data;
    let lo_data;
    unsafe {
        tx_data = Vec::from_raw_parts(tx_json_ptr, tx_json_size, tx_json_size);
        lo_data = Vec::from_raw_parts(lo_json_ptr, lo_json_size, lo_json_size);
    }

    log("Parsing transaction JSON");
    let tx_json_value: Value = match serde_json::from_slice(tx_data.as_slice()) {
        Ok(v) => v,
        Err(e) => {
            log(&format!("Error parsing transaction JSON: {:?}", e));
            return false;
        }
    };
    
    log("Parsing ledger object JSON");
    let lo_json_value: Value = match serde_json::from_slice(lo_data.as_slice()) {
        Ok(v) => v,
        Err(e) => {
            log(&format!("Error parsing ledger object JSON: {:?}", e));
            return false;
        }
    };
    
    log("Extracting Account fields");
    let tx_account = match tx_json_value.get("Account") {
        Some(v) => {
            log(&format!("Transaction Account: {}", v));
            v
        },
        None => {
            log("Transaction JSON has no Account field");
            return false;
        }
    };
    
    let lo_account = match lo_json_value.get("Account") {
        Some(v) => {
            log(&format!("Ledger Object Account: {}", v));
            v
        },
        None => {
            log("Ledger Object JSON has no Account field");
            return false;
        }
    };
    
    let result = tx_account == lo_account;
    log(&format!("Account comparison result: {}", result));
    result
}

// Simple logging function that does nothing in WASM
// In a real environment, this would be connected to the host's logging system
fn log(message: &str) {
    // This function does nothing in WASM but is here for documentation
    // In WebAssembly, we typically need to use host-provided functions for logging
    #[cfg(target_arch = "wasm32")]
    {
        // The empty block is intentional - we can't log directly from WASM
    }
    
    // When not compiled to WASM (e.g., for testing), we can use standard Rust println
    #[cfg(not(target_arch = "wasm32"))]
    println!("[WASM] {}", message);
} 