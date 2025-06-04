#![no_std]

use xrpl_std::get_ledger_obj_nested_field;
use xrpl_std::keylet::oracle_keylet;
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield;
use xrpl_std::types::AccountID;

const ORACLE_OWNER: &AccountID = b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3";
const ORACLE_DOCUMENT_ID: i32 = 1;

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub fn get_price_from_oracle(slot: i32) -> Option<u64> {
    let mut locator = LocatorPacker::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);
    let asset_price = match get_ledger_obj_nested_field(slot, &locator) {
        Some(v) => v,
        None => return None,
    };

    return Some(get_u64_from_buffer(&asset_price[0..8]));
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> bool {
    let oracle_keylet = match oracle_keylet(ORACLE_OWNER, ORACLE_DOCUMENT_ID) {
        Some(v) => v,
        None => return false,
    };

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
