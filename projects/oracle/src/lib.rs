#![no_std]

use xrpl_std::core::ledger_objects::ledger_object;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::keylets::oracle_keylet_safe;
use xrpl_std::host::trace::trace_num;
use xrpl_std::host::{Result::Err, Result::Ok};
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield;

const ORACLE_OWNER: AccountID =
    AccountID(*b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3");
const ORACLE_DOCUMENT_ID: i32 = 1;

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
    let mut locator = LocatorPacker::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);
    let asset_price = match ledger_object::get_nested_field(slot, &locator) {
        Ok(contract_data) => match contract_data {
            Some(data) => get_u64_from_buffer(&data[0..8]),
            None => {
                return None; // Must return to short circuit.
            }
        },
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
