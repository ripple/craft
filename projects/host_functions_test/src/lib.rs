#![no_std]
#![allow(unused_imports)]

// Test multiple categories of host functions:
// 1. High-level transaction access (get_account)
// 2. Direct host function access (get_tx_field) 
// 3. Ledger data access (get_account_balance)
// 4. Tracing and debugging

use xrpl_std::host::trace::{trace_data, trace_num, DataRepr};
use xrpl_std::host::get_tx_field;  // Direct host function access
use xrpl_std::sfield::{SignerEntries, SignerEntry, SignerWeight};
use xrpl_std::get_account_balance;
use xrpl_std::core::tx::current_transaction::get_account;
use xrpl_std::core::field_codes::{SF_ACCOUNT, SF_FEE, SF_SEQUENCE, SF_FLAGS};

#[unsafe(no_mangle)]
pub extern "C" fn finish(_reserved: u32) -> i32 {
    // Test 1: High-level transaction field access
    let account_id_tx = get_account();
    let _ = trace_data("High-level Account:", &account_id_tx.0, DataRepr::AsHex);
    
    // Test 2: Direct host function access to same field
    let mut raw_account_buffer = [0u8; 20];
    let account_bytes_read = unsafe {
        get_tx_field(SF_ACCOUNT, raw_account_buffer.as_mut_ptr(), raw_account_buffer.len())
    };
    
    if account_bytes_read < 0 {
        let _ = trace_num("ERROR: get_tx_field(Account) failed:", account_bytes_read as i64);
        return -1;
    }
    
    let _ = trace_data("Raw Account via get_tx_field:", &raw_account_buffer, DataRepr::AsHex);
    let _ = trace_num("Account field bytes read:", account_bytes_read as i64);
    
    // Test 3: More direct host function calls for different field types
    let mut fee_buffer = [0u8; 8]; 
    let fee_bytes_read = unsafe {
        get_tx_field(SF_FEE, fee_buffer.as_mut_ptr(), fee_buffer.len())
    };
    
    if fee_bytes_read > 0 {
        let fee_value = u64::from_le_bytes(fee_buffer);
        let _ = trace_num("Fee from get_tx_field:", fee_value as i64);
    }
    
    let mut sequence_buffer = [0u8; 4];
    let seq_bytes_read = unsafe {
        get_tx_field(SF_SEQUENCE, sequence_buffer.as_mut_ptr(), sequence_buffer.len())
    };
    
    if seq_bytes_read > 0 {
        let sequence_value = u32::from_le_bytes(sequence_buffer);
        let _ = trace_num("Sequence from get_tx_field:", sequence_value as i64);
    }
    
    // Test 4: Ledger access (original test)
    let balance = match get_account_balance(&account_id_tx) {
        Some(v) => v,
        None => {
            let _ = trace_data("ERROR: Could not get balance for account", &[], DataRepr::AsHex);
            return -5;
        }
    };
    
    let _ = trace_num("Account Balance:", balance as i64);
    
    if balance <= 0 {
        let _ = trace_data("ERROR: Account has zero or negative balance", &[], DataRepr::AsHex);
        return -9;
    }
    
    // All tests passed
    let _ = trace_data("SUCCESS: All host function tests completed", &[], DataRepr::AsHex);
    1
}