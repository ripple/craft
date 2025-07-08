#![no_std]

use xrpl_std::core::error_codes::match_result_code;
use xrpl_std::core::locator::Locator;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::keylets::oracle_keylet_safe;
use xrpl_std::host::trace::trace_num;
use xrpl_std::host::{Result::Err, Result::Ok};
use xrpl_std::{host, sfield};

const ORACLE_OWNER: AccountID =
    AccountID(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
const ORACLE_DOCUMENT_ID: i32 = 1;

// TODO: Update this function to handle errors and return a Result<u64> instead.
pub fn get_u64_from_buffer(bytes: &[u8]) -> u64 {
    let mut result: u64 = 0;

    // rippled uses big-endian: most significant byte is first
    let mut i = 0;
    while i < 8 {
        result = (result << 8) | (bytes[i] as u64);
        i += 1;
    }

    result
}

pub fn get_price_from_oracle(slot: i32) -> Option<u64> {
    let mut locator = Locator::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);

    let mut data: [u8; 8] = [0; 8];
    let result_code = unsafe {
        host::get_ledger_obj_nested_field(
            slot,
            locator.get_addr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        )
    };
    let asset_price = match match_result_code(result_code, || data) {
        Ok(asset_bytes) => get_u64_from_buffer(&asset_bytes[0..8]),
        Err(error) => {
            let _ = trace_num("Error getting asset_price", error.code() as i64);
            return None; // Must return to short circuit.
        }
    };
    Some(asset_price)
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let oracle_keylet = oracle_keylet_safe(&ORACLE_OWNER, ORACLE_DOCUMENT_ID);

    let slot: i32;
    unsafe {
        slot = xrpl_std::host::cache_ledger_obj(oracle_keylet.as_ptr(), oracle_keylet.len(), 0);
        if slot < 0 {
            return false;
        };
    }

    let price = match get_price_from_oracle(slot) {
        Some(v) => v,
        None => return false,
    };

    price > 1
}
