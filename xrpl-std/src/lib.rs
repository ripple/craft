pub mod sfield;
pub mod locator;
pub mod host_lib;

pub const XRPL_ACCOUNT_ID_SIZE: usize = 40; //TODO size, after json to binary PR
pub const XRPL_KEYLET_SIZE: usize = 68; //TODO size, after json to binary PR
pub const XRPL_HASH256_SIZE: usize = 64; //TODO size, after json to binary PR
pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096; //TODO size??
pub type AccountID = [u8; XRPL_ACCOUNT_ID_SIZE];
pub type Keylet = [u8; XRPL_KEYLET_SIZE];
pub type Hash256 = [u8; XRPL_HASH256_SIZE];
pub type ContractData = [u8; XRPL_CONTRACT_DATA_SIZE];

pub fn get_tx_account_id() -> Option<AccountID> { //TODO replace with Tx Object
    let mut account_id: AccountID = [0; XRPL_ACCOUNT_ID_SIZE];
    if unsafe {
        host_lib::get_tx_field(sfield::Account, account_id.as_mut_ptr() as i32, account_id.len() as i32)
    } == XRPL_ACCOUNT_ID_SIZE as i32 {
        Some(account_id)
    } else {
        None
    }
}

pub fn get_current_escrow_account_id() -> Option<AccountID> { //TODO replace with escrow Object
    let mut account_id: AccountID = [0; XRPL_ACCOUNT_ID_SIZE];
    if unsafe {
        host_lib::get_current_ledger_obj_field(sfield::Account, account_id.as_mut_ptr() as i32, account_id.len() as i32)
    } == XRPL_ACCOUNT_ID_SIZE as i32 {
        Some(account_id)
    } else {
        None
    }
}

pub fn get_current_escrow_destination() -> Option<AccountID> { //TODO replace with escrow Object
    let mut account_id: AccountID = [0; XRPL_ACCOUNT_ID_SIZE];
    if unsafe {
        host_lib::get_current_ledger_obj_field(sfield::Destination, account_id.as_mut_ptr() as i32, account_id.len() as i32)
    } == XRPL_ACCOUNT_ID_SIZE as i32 {
        Some(account_id)
    } else {
        None
    }
}

pub fn get_current_escrow_data() -> Option<ContractData> { //TODO replace with escrow Object
    let mut data: ContractData = [0; XRPL_CONTRACT_DATA_SIZE];
    if unsafe {
        host_lib::get_current_ledger_obj_field(sfield::Data, data.as_mut_ptr() as i32, data.len() as i32)
    } == XRPL_ACCOUNT_ID_SIZE as i32 {
        Some(data)
    } else {
        None
    }
}

pub fn get_current_escrow_finish_after() -> Option<i32> { //TODO replace with escrow Object
    let mut after = 0i32;
    if unsafe {
        host_lib::get_current_ledger_obj_field(sfield::Data, (&mut after) as *mut i32 as i32, 4)
    } == XRPL_ACCOUNT_ID_SIZE as i32 {
        Some(after)
    } else {
        None
    }
}

pub fn get_account_balance(aid: &AccountID) -> Option<u64> { //TODO replace with accountRoot
    let keylet = match account_keylet(aid) {
        None => { return None }
        Some(keylet) => { keylet }
    };
    let slot = unsafe {
        host_lib::ledger_slot_set(keylet.as_ptr() as i32, keylet.len() as i32, 0)
    };
    if slot <= 0 {
        return None;
    }
    let mut balance = 0u64;
    if unsafe {
        host_lib::get_ledger_obj_field(slot, sfield::Balance, (&mut balance) as *mut u64 as i32, 8)
    } == 8 {
        Some(balance)
    } else {
        None
    }
}

pub fn account_keylet(aid: &AccountID) -> Option<Keylet> {
    let mut key_let: Keylet = [0; XRPL_KEYLET_SIZE];
    if XRPL_KEYLET_SIZE as i32 == unsafe {
        host_lib::accountKeylet(aid.as_ptr() as i32, aid.len() as i32, key_let.as_mut_ptr() as i32, key_let.len() as i32)
    } {
        Some(key_let)
    } else {
        None
    }
}

pub fn update_current_escrow_data(data: ContractData) {
    unsafe {
        host_lib::updateData(data.as_ptr() as i32, data.len() as i32);
    }
}