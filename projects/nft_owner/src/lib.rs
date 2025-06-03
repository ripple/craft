#![no_std]

use xrpl_std::host::trace::{trace_data, DataRepr};
use xrpl_std::{get_current_escrow_destination, get_first_memo, get_nft, XRPL_NFTID_SIZE};

#[no_mangle]
pub extern "C" fn finish() -> bool {
    unsafe {
        let memo = match get_first_memo() {
            Some(v) => v,
            None => return false,
        };

        trace_data("Memo (NFT)", &memo, DataRepr::AsHex);
        let nft: [u8; XRPL_NFTID_SIZE] = memo[0..32].try_into().unwrap();

        trace_data("Memo (NFT2)", &nft, DataRepr::AsHex);

        let destination = match get_current_escrow_destination() {
            Some(v) => v,
            None => return false,
        };

        match get_nft(&destination, &nft) {
            Some(v) => return true,
            None => return false,
        };
    }
}
