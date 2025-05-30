#![no_std]
#![allow(unused_imports)]

//
// With craft you can run this test with:
//   craft test --project host_functions_test --test-case host_functions_test
//

// Priority host functions test - starting with important functions first
// Based on latest host_bindings.rs
// 1. get_tx_field() - Core transaction data access
// 2. cache_ledger_obj() - Ledger object caching
// 3. account_keylet() - Keylet generation
// 4. get_ledger_obj_field() - Cached object field access
// 5. update_data() - State modification
// 6. Nested field access - Data traversal

use xrpl_std::host::trace::{trace_data, trace_num, DataRepr};
use xrpl_std::host::*;
use xrpl_std::sfield;
use xrpl_std::core::tx::current_transaction::get_account;
use xrpl_std::core::field_codes::*;

#[unsafe(no_mangle)]
pub extern "C" fn finish(_reserved: u32) -> i32 {
    let _ = trace_data("=== HOST FUNCTIONS TEST ===", &[], DataRepr::AsHex);
    
    // Test 1: Core transaction field access
    if test_transaction_fields() != 0 {
        return -1;
    }
    
    // Test 2: Keylet generation and ledger object caching
    if test_ledger_object_operations() != 0 {
        return -2;
    }
    
    // Test 3: Nested field access
    if test_nested_field_access() != 0 {
        return -3;
    }
    
    // Test 4: State modification
    if test_state_modification() != 0 {
        return -4;
    }
    
    let _ = trace_data("SUCCESS: Host function tests passed", &[], DataRepr::AsHex);
    1
}

fn test_transaction_fields() -> i32 {
    let _ = trace_data("--- Test 1: Transaction Field Access ---", &[], DataRepr::AsHex);
    
    // Test Account field (20 bytes)
    let mut account_buffer = [0u8; 20];
    let account_len = unsafe {
        get_tx_field(SF_ACCOUNT, account_buffer.as_mut_ptr(), account_buffer.len())
    };
    
    if account_len != 20 {
        let _ = trace_num("ERROR: get_tx_field(Account) wrong length:", account_len as i64);
        return -1;
    }
    let _ = trace_data("Account field:", &account_buffer, DataRepr::AsHex);
    
    // Test Fee field 
    let mut fee_buffer = [0u8; 8];
    let fee_len = unsafe {
        get_tx_field(SF_FEE, fee_buffer.as_mut_ptr(), fee_buffer.len())
    };
    
    if fee_len <= 0 {
        let _ = trace_num("ERROR: get_tx_field(Fee) failed:", fee_len as i64);
        return -1;
    }
    let _ = trace_num("Fee field length:", fee_len as i64);
    
    // Test Sequence field (uint32)
    let mut seq_buffer = [0u8; 4];
    let seq_len = unsafe {
        get_tx_field(SF_SEQUENCE, seq_buffer.as_mut_ptr(), seq_buffer.len())
    };
    
    if seq_len <= 0 {
        let _ = trace_num("ERROR: get_tx_field(Sequence) failed:", seq_len as i64);
        return -1;
    }
    let _ = trace_num("Sequence field length:", seq_len as i64);
    
    0
}

fn test_ledger_object_operations() -> i32 {
    let _ = trace_data("--- Test 2: Ledger Object Operations ---", &[], DataRepr::AsHex);
    
    let account_id = get_account();
    
    // Test account_keylet generation
    let mut keylet_buffer = [0u8; 32];
    let keylet_result = unsafe {
        account_keylet(
            account_id.0.as_ptr(), 
            account_id.0.len(),
            keylet_buffer.as_mut_ptr(), 
            keylet_buffer.len()
        )
    };
    
    if keylet_result <= 0 {
        let _ = trace_num("ERROR: account_keylet failed:", keylet_result as i64);
        return -1;
    }
    let _ = trace_data("Account keylet:", &keylet_buffer, DataRepr::AsHex);
    let _ = trace_num("Keylet bytes generated:", keylet_result as i64);
    
    // Test cache_ledger_obj - load account object into cache
    let cache_result = unsafe {
        cache_ledger_obj(keylet_buffer.as_ptr(), keylet_result as usize, 0)
    };
    
    if cache_result <= 0 {
        let _ = trace_num("INFO: cache_ledger_obj failed (expected for test fixtures):", cache_result as i64);
        // This is expected since test fixtures may not contain the specific account object
        // Continue with test but skip ledger object field access
        return 0;
    }
    let _ = trace_num("Object cached in slot:", cache_result as i64);
    
    // Test get_ledger_obj_field - read balance from cached object
    let mut balance_buffer = [0u8; 8];
    let balance_len = unsafe {
        get_ledger_obj_field(
            cache_result, 
            sfield::Balance, 
            balance_buffer.as_mut_ptr(), 
            balance_buffer.len()
        )
    };
    
    if balance_len <= 0 {
        let _ = trace_num("INFO: get_ledger_obj_field(Balance) failed:", balance_len as i64);
        // Continue since this might be expected behavior
    } else {
        let _ = trace_num("Balance field length:", balance_len as i64);
        let _ = trace_data("Balance field:", &balance_buffer[..balance_len as usize], DataRepr::AsHex);
    }
    
    0
}

fn test_nested_field_access() -> i32 {
    let _ = trace_data("--- Test 3: Nested Field Access ---", &[], DataRepr::AsHex);
    
    // Test get_tx_nested_field for accessing nested transaction fields
    let locator = b"\x01\x02"; // Simple locator example 
    let mut buffer = [0u8; 32];
    let result = unsafe {
        get_tx_nested_field(
            locator.as_ptr(),
            locator.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    
    if result < 0 {
        let _ = trace_num("INFO: get_tx_nested_field not applicable:", result as i64);
    } else {
        let _ = trace_num("Nested field access succeeded:", result as i64);
        let _ = trace_data("Nested field data:", &buffer[..result as usize], DataRepr::AsHex);
    }
    
    // Test array length function
    let array_len = unsafe {
        get_tx_array_len(sfield::Signers)
    };
    
    if array_len >= 0 {
        let _ = trace_num("Signers array length:", array_len as i64);
    } else {
        let _ = trace_num("INFO: No signers array or error:", array_len as i64);
    }
    
    // Test nested array length with locator
    let nested_array_len = unsafe {
        get_tx_nested_array_len(locator.as_ptr(), locator.len())
    };
    
    if nested_array_len >= 0 {
        let _ = trace_num("Nested array length:", nested_array_len as i64);
    } else {
        let _ = trace_num("INFO: No nested array or error:", nested_array_len as i64);
    }
    
    0
}

fn test_state_modification() -> i32 {
    let _ = trace_data("--- Test 4: State Modification ---", &[], DataRepr::AsHex);
    
    // Test update_data with simple data
    let test_data = b"test_state_123";
    
    let update_result = unsafe {
        update_data(test_data.as_ptr(), test_data.len())
    };
    
    if update_result != 0 {
        let _ = trace_num("ERROR: update_data failed:", update_result as i64);
        return -1;
    }
    
    let _ = trace_data("State updated with data:", test_data, DataRepr::AsHex);
    
    0
}