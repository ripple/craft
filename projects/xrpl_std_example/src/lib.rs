use xrpl_std::{account_keylet, get_tx_account_id, print_data};
// use xrpl_std::{
//     account_keylet, credential_keylet, escrow_keylet, get_tx_account_id, oracle_keylet, print_data,
//     print_number,
// };
pub use xrpl_std::{allocate, deallocate};

#[no_mangle]
pub extern "C" fn ready() -> bool {
    unsafe {
        let sender = get_tx_account_id();
        let account_keylet = account_keylet(&sender);

        print_data(&account_keylet);

        true
    }
}
