#![no_std]

use xrpl_std::get_ledger_obj_nested_field;
use xrpl_std::host::trace::{trace_data, DataRepr};
use xrpl_std::keylet::oracle_keylet;
use xrpl_std::locator::LocatorPacker;
use xrpl_std::sfield;
use xrpl_std::types::{AccountID, ContractData};

const ORACLE_OWNER: &AccountID = b"\xd5\xb9\x84VP\x9f \xb5'\x9d\x1eJ.\xe8\xb2\xaa\x82\xaec\xe3";

#[no_mangle]
pub fn get_price_from_oracle(slot: i32) -> Option<ContractData> {
    let mut locator = LocatorPacker::new();
    locator.pack(sfield::PriceDataSeries);
    locator.pack(0);
    locator.pack(sfield::AssetPrice);
    let asset_price = match get_ledger_obj_nested_field(slot, &locator) {
        Some(v) => v,
        None => return None,
    };

    trace_data("asset_price", &asset_price, DataRepr::AsHex);

    return Some(asset_price);
}

#[no_mangle]
pub extern "C" fn finish() -> bool {
    let oracle_keylet = match oracle_keylet(ORACLE_OWNER, 1) {
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

    trace_data("price", &price, DataRepr::AsHex);

    false
}
