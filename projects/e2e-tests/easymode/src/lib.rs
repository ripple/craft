#![allow(unused_imports)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::current_tx::contract_call::{get_current_contract_call, ContractCall};
use xrpl_wasm_std::core::current_tx::traits::{ContractCallFields, TransactionCommonFields};
use xrpl_wasm_std::core::data::codec::{get_data, set_data};
use xrpl_wasm_std::core::params::function::{get_function_param, safe_get_function_param};
use xrpl_wasm_std::core::params::instance::get_instance_param;
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::token_amount::TokenAmount;
use xrpl_wasm_std::core::types::number::Number;
use xrpl_wasm_std::host::trace::{trace_num, DataRepr};
use xrpl_wasm_std::host::{function_param, instance_param};

const SUCCESS: i32 = 0;
const BAD_PARAM: i32 = -1;
const MAX_LIMIT: i32 = -2;

#[unsafe(no_mangle)]
pub extern "C" fn topup() -> i32 {
    // TODO: Validate XRP
    SUCCESS
}

#[unsafe(no_mangle)]
pub extern "C" fn faucet() -> i32 {
    // Get Incoming Transaction (ContractCall)
    let contract_call: ContractCall = get_current_contract_call();

    // Get: Incoming Account
    let otxn_account = contract_call.get_account().unwrap();

    // Get: Contract Account
    let contract_account = contract_call.get_contract_account().unwrap();

    // Get: u32
    let max_limit = match get_instance_param::<u32>(0) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("`max_limit` Parameter Error Code:", err as i64);
            return BAD_PARAM;
        }
    };

    // Get: TokenAmount
    let amount = match get_instance_param::<TokenAmount>(1) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("`amount` Parameter Error Code:", err as i64);
            return BAD_PARAM;
        }
    };

    // Read limit
    let current_limit = match get_data::<u32>(&contract_account, &otxn_account.to_hex_bytes()[..]) {
        Some(limit) => limit,
        None => {
            let _ = trace_num("`current_limit` not found", BAD_PARAM as i64);
            return BAD_PARAM;
        },
    };

    // Validate limit before proceeding
    if current_limit >= max_limit {
        let _ = trace_num("Exceeded max limit:", max_limit.into());
        return MAX_LIMIT;
    }

    // Get: AccountID
    let account = safe_get_function_param::<AccountID>(0);

    // Transfer: from the "contract" to the "account"
    let tx_id = amount.transfer(&account);
    if tx_id < 0 {
        let _ = trace_num("AMOUNT Transfer Error Code:", tx_id as i64);
        return tx_id;
    }

    // Update limit
    let new_limit = current_limit + 1;
    if let Err(e) = set_data::<u32>(
        &contract_account,
        &otxn_account.to_hex_bytes()[..],
        new_limit,
    ) {
        let _ = trace_num("Set Limit Error Code:", e as i64);
        return e;
    }

    return SUCCESS;
}
