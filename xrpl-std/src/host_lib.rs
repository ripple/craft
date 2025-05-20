#[link(wasm_import_module = "host_lib")]
extern "C" {
    pub fn get_ledger_sqn(out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_parent_ledger_time(out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_parent_ledger_hash(out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;

    pub fn ledger_slot_set(keylet_ptr: * const u8, keylet_len: usize, slot_num: i32) -> i32;

    pub fn get_tx_field(field: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_tx_field2(field: i32, field2: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_tx_field3(field: i32, field2: i32, field3: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_tx_field4(field: i32, field2: i32, field3: i32, field4: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_tx_field5(field: i32, field2: i32, field3: i32, field4: i32, field5: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_tx_field6(field: i32, field2: i32, field3: i32, field4: i32, field5: i32, field6: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    
    pub fn get_current_ledger_obj_field(field: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_ledger_obj_field(slot: i32, field: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;

    pub fn get_tx_nested_field(locator_ptr: * const u8, locator_len: usize, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_current_ledger_obj_nested_field(locator_ptr: * const u8, locator_len: usize, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn get_ledger_obj_nested_field(slot: i32, locator_ptr: * const u8, locator_len: usize, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;

    pub fn get_tx_array_len(field: i32) -> i32;
    pub fn get_current_ledger_obj_array_len(field: i32) -> i32;
    pub fn get_ledger_obj_array_len(slot: i32, field: i32) -> i32;

    pub fn get_tx_nested_array_len(locator_ptr: * const u8, locator_len: usize) -> i32;
    pub fn get_current_ledger_obj_nested_array_len(locator_ptr: * const u8, locator_len: usize) -> i32;
    pub fn get_ledger_obj_nested_array_len(slot: i32, locator_ptr: * const u8, locator_len: usize) -> i32;

    pub fn update_data(data_ptr: * const u8, data_len: usize);

    pub fn compute_sha512_half(data_ptr: * const u8, data_len: usize, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    
    pub fn account_keylet(account_ptr: * const u8, account_len: usize, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    
    pub fn credential_keylet(
        subject_ptr: i32, subject_len: i32,
        issuer_ptr: i32, issuer_len: i32,
        cred_type_ptr: i32, cred_type_len: i32,
        out_buff_ptr: *mut u8, out_buff_len: usize,
    ) -> i32;
    pub fn escrow_keylet(account_ptr: * const u8, account_len: usize, sequence: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    pub fn oracle_keylet(account_ptr: * const u8, account_len: usize, document_id: i32, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;

    pub fn get_NFT(owner_ptr: * const u8, owner_len: usize, nft_id_ptr: * const u8, nft_id_len: usize, out_buff_ptr: * mut u8, out_buff_len: usize) -> i32;
    
    pub fn print(data_ptr: * const u8, data_len: usize); //TODO replace with David's trace
}
