#[link(wasm_import_module = "host_lib")]
extern "C" {
    pub fn getLedgerSqn() -> i32;
    pub fn getParentLedgerTime() -> i32;
    //TODO should we change all ptr to  *mut u8?
    pub fn get_parent_ledger_hash(out_buff_ptr: i32, out_buff_len: i32) -> i32;
    
    pub fn ledger_slot_set(keylet_ptr: i32, keylet_len: i32, slot_num: i32) -> i32;

    pub fn get_tx_field(field: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;
    pub fn get_current_ledger_obj_field(field: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;
    pub fn get_ledger_obj_field(slot: i32, field: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;

    pub fn get_tx_nested_field(locator_ptr: i32, locator_len: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;
    pub fn get_current_ledger_obj_nested_field(locator_ptr: i32, locator_len: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;
    pub fn get_ledger_obj_nested_field(slot: i32, locator_ptr: i32, locator_len: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;

    pub fn get_tx_field_len(field: i32) -> i32;
    pub fn get_current_ledger_obj_field_len(field: i32) -> i32;
    pub fn get_ledger_obj_field_len(slot: i32, field: i32) -> i32;

    pub fn get_tx_nested_field_len(locator_ptr: i32, locator_len: i32) -> i32;
    pub fn get_current_ledger_obj_nested_field_len(locator_ptr: i32, locator_len: i32) -> i32;
    pub fn get_ledger_obj_nested_field_len(slot: i32, locator_ptr: i32, locator_len: i32) -> i32;
    
    pub fn updateData(data_ptr: i32, data_len: i32);

    pub fn computeSha512HalfHash(data_ptr: i32, data_len: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;
    pub fn accountKeylet(account_ptr: i32, account_len: i32, out_buff_ptr: i32, out_buff_len: i32) -> i32;
    
    //TODO the following will be in a separate PR
    pub fn credentialKeylet(
        subject_ptr: i32,
        subject_len: i32,
        issuer_ptr: i32,
        issuer_len: i32,
        cred_type_ptr: i32,
        cred_type_len: i32,
    ) -> i32; //TODO add output
    pub fn escrowKeylet(account_ptr: i32, account_len: i32, sequence: i32) -> i32; //TODO add output
    pub fn oracleKeylet(account_ptr: i32, account_len: i32, document_id: i32) -> i32; //TODO add output
    pub fn getNFT(account_ptr: i32, account_len: i32, nft_id_ptr: i32, nft_id_len: i32) -> i32; //TODO add output
    pub fn print(str_ptr: i32, str_len: i32); //TODO replace with trace
}
