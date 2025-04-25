use std::mem;

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    println!("allocate {:?}", pointer);
    pointer
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut u8, capacity: usize) {
    unsafe {
        println!("deallocate {:?}", pointer);
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

pub type AccountID = Vec<u8>;

pub mod host_lib {
    #[link(wasm_import_module = "host_lib")]
    extern "C" {
        // fetch ledger data
        pub fn getLedgerSqn() -> i32;
        pub fn getParentLedgerTime() -> i32;
        pub fn getTxField(fname_ptr: i32, fname_len: i32) -> i32;
        pub fn getLedgerEntryField(
            le_type: i32,
            key_ptr: i32,
            key_len: i32,
            fname_ptr: i32,
            fname_len: i32,
        ) -> i32;
        pub fn getCurrentLedgerEntryField(fname_ptr: i32, fname_len: i32) -> i32;
        pub fn getNFT(account_ptr: i32, account_len: i32, nft_id_ptr: i32, nft_id_len: i32) -> i32;

        // update ledger data
        pub fn updateData(data_ptr: i32, data_len: i32);

        // utils
        pub fn computeSha512HalfHash(data_ptr: i32, data_len: i32) -> i32;
        pub fn accountKeylet(account_ptr: i32, account_len: i32) -> i32;
        pub fn credentialKeylet(
            subject_ptr: i32,
            subject_len: i32,
            issuer_ptr: i32,
            issuer_len: i32,
            cred_type_ptr: i32,
            cred_type_len: i32,
        ) -> i32;
        pub fn escrowKeylet(account_ptr: i32, account_len: i32, sequence: i32) -> i32;
        pub fn oracleKeylet(account_ptr: i32, account_len: i32, document_id: i32) -> i32;
        pub fn print(str_ptr: i32, str_len: i32);
    }
}

unsafe fn read_data(ptr: i32) -> Vec<u8> {
    let int_buf = Vec::from_raw_parts(ptr as *mut u8, 8, 8);
    let mut ptr_array: [u8; 4] = [0; 4];
    let mut len_array: [u8; 4] = [0; 4];
    ptr_array.clone_from_slice(&int_buf[0..4]);
    len_array.clone_from_slice(&int_buf[4..8]);
    let ptr = i32::from_le_bytes(ptr_array);
    let len = i32::from_le_bytes(len_array);
    Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize)
}

unsafe fn read_string(ptr: i32) -> String {
    let mut ptr_array: [u8; 4] = [0; 4];
    let mut len_array: [u8; 4] = [0; 4];
    let int_buf = Vec::from_raw_parts(ptr as *mut u8, 8, 8);
    ptr_array.clone_from_slice(&int_buf[0..4]);
    len_array.clone_from_slice(&int_buf[4..8]);
    let ptr = i32::from_le_bytes(ptr_array);
    let len = i32::from_le_bytes(len_array);
    String::from_raw_parts(ptr as *mut u8, len as usize, len as usize)
}

pub unsafe fn get_tx_account_id() -> AccountID {
    let mut fname = String::from("Account");
    let pointer = fname.as_mut_ptr();
    let len = fname.len();
    let r_ptr = host_lib::getTxField(pointer as i32, len as i32);
    // assert_eq!(r_len, 20);
    let r = read_data(r_ptr);
    AccountID::from(r)
}

pub unsafe fn get_current_escrow_account_id() -> AccountID {
    let mut fname = String::from("Account");
    let pointer = fname.as_mut_ptr();
    let len = fname.len();
    let r_ptr = host_lib::getCurrentLedgerEntryField(pointer as i32, len as i32);
    // assert_eq!(r_len, 20);
    let r = read_data(r_ptr);
    AccountID::from(r)
}

pub unsafe fn get_current_escrow_destination() -> AccountID {
    let mut fname = String::from("Destination");
    let pointer = fname.as_mut_ptr();
    let len = fname.len();
    let r_ptr = host_lib::getCurrentLedgerEntryField(pointer as i32, len as i32);
    // assert_eq!(r_len, 20);
    let r = read_data(r_ptr);
    AccountID::from(r)
}

pub unsafe fn get_current_escrow_data() -> Vec<u8> {
    let mut fname = String::from("Data");
    let pointer = fname.as_mut_ptr();
    let len = fname.len();
    let r_ptr = host_lib::getCurrentLedgerEntryField(pointer as i32, len as i32);
    // assert_eq!(r_len, 20);
    read_data(r_ptr)
}

pub unsafe fn get_current_escrow_finish_after() -> i32 {
    let mut fname = String::from("FinishAfter");
    let pointer = fname.as_mut_ptr();
    let len = fname.len();
    let r_ptr = host_lib::getCurrentLedgerEntryField(pointer as i32, len as i32);
    // assert_eq!(r_len, 20);
    let r = read_string(r_ptr);
    r.parse::<i32>().unwrap()
}

pub unsafe fn get_current_escrow_cancel_after() -> i32 {
    let mut fname = String::from("CancelAfter");
    let pointer = fname.as_mut_ptr();
    let len = fname.len();
    let r_ptr = host_lib::getCurrentLedgerEntryField(pointer as i32, len as i32);
    // assert_eq!(r_len, 20);
    let r = read_string(r_ptr);
    r.parse::<i32>().unwrap()
}

pub unsafe fn get_account_balance(aid: &AccountID) -> u64 {
    let key_ptr = aid.as_ptr();
    let key_len = aid.len();
    let mut fname = String::from("Balance");
    let fname_ptr = fname.as_mut_ptr();
    let fname_len = fname.len();
    let r_ptr = host_lib::getLedgerEntryField(
        0x0061,
        key_ptr as i32,
        key_len as i32,
        fname_ptr as i32,
        fname_len as i32,
    );
    let r = read_string(r_ptr);
    r.parse::<u64>().unwrap()
}

pub unsafe fn update_current_escrow_data(data: Vec<u8>) {
    let pointer = data.as_ptr();
    let len = data.len();
    host_lib::updateData(pointer as i32, len as i32);
}

pub unsafe fn print_data(s: &Vec<u8>) {
    let s_ptr = s.as_ptr();
    let s_len = s.len();
    host_lib::print(s_ptr as i32, s_len as i32);
}

pub unsafe fn print_number<T: ToString>(number: &T) {
    let s = number.to_string();
    let s_ptr = s.as_ptr();
    let s_len = s.len();
    host_lib::print(s_ptr as i32, s_len as i32);
}
