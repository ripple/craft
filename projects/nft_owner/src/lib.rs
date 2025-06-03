#![no_std]

use xrpl_std::host::get_tx_nested_field;
use xrpl_std::host::trace::{trace_data, trace_num, DataRepr};
use xrpl_std::{
    get_current_escrow_destination, get_nft, ContractData, XRPL_CONTRACT_DATA_SIZE, XRPL_NFTID_SIZE,
};
use xrpl_std::{locator, sfield};

#[no_mangle]
pub fn get_first_memo() -> Option<ContractData> {
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    let mut locator = locator::LocatorPacker::new();
    locator.pack(sfield::Memos);
    locator.pack(0);
    locator.pack(sfield::MemoData);
    unsafe {
        let retcode = get_tx_nested_field(
            locator.get_addr(),
            locator.num_packed_bytes(),
            data.as_mut_ptr(),
            data.len(),
        );
        if retcode > 0 {
            Some(data)
        } else {
            trace_num("Memo (first)", i64::from(retcode));
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn finish() -> bool {
    let memo = match get_first_memo() {
        Some(v) => v,
        None => return false,
    };

    let nft: [u8; XRPL_NFTID_SIZE] = memo[0..32].try_into().unwrap();

    let destination = match get_current_escrow_destination() {
        Some(v) => v,
        None => return false,
    };

    match get_nft(&destination, &nft) {
        Some(_v) => return true,
        None => return false,
    };
}
