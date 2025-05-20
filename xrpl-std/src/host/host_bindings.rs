#[link(wasm_import_module = "host_lib")]
unsafe extern "C" {
    pub fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    pub fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    pub fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    pub fn ledger_slot_set(keylet_ptr: *const u8, keylet_len: usize, slot_num: i32) -> i32;

    pub fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    pub fn get_current_ledger_obj_field(
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_ledger_obj_field(
        slot: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    pub fn get_tx_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_current_ledger_obj_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_ledger_obj_nested_field(
        slot: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    pub fn get_tx_array_len(field: i32) -> i32;
    pub fn get_current_ledger_obj_array_len(field: i32) -> i32;
    pub fn get_ledger_obj_array_len(slot: i32, field: i32) -> i32;

    pub fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;
    pub fn get_current_ledger_obj_nested_array_len(
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;
    pub fn get_ledger_obj_nested_array_len(
        slot: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    pub fn update_data(data_ptr: *const u8, data_len: usize);

    pub fn compute_sha512_half(
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    pub fn account_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    pub fn credential_keylet(
        subject_ptr: i32,
        subject_len: i32,
        issuer_ptr: i32,
        issuer_len: i32,
        cred_type_ptr: i32,
        cred_type_len: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn escrow_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn oracle_keylet(
        account_ptr: *const u8,
        account_len: usize,
        document_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    //TODO the following will be in separate PRs
    pub fn get_NFT(
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    // #############################
    // Host Function Category: TRACE
    // #############################

    /// Print to the trace log on XRPLd. Any XRPLd instance set to \"trace\" log level will see this.
    ///
    /// # Parameters
    /// * `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// * `msg_read_len`: The byte length of the text to send to the trace log.
    /// * `data_read_ptr`: A pointer to an array of bytes containing arbitrary data.
    /// * `data_read_len`: The byte length of the data to send to the trace log.
    /// * `as_hex`: If 0 treat the data_read_ptr as pointing at a string of text, otherwise treat it
    ///      as data and print hex.
    ///
    /// # Returns
    ///
    /// Returns an integer representing the result of the operation. A value of `0` signifies
    /// success. Non-zero values indicate an error (e.g., incorrect buffer sizes).
    pub fn trace(
        msg_read_ptr: u32,
        msg_read_len: usize,
        data_read_ptr: u32,
        data_read_len: usize,
        as_hex: u32,
    ) -> i32;

    /// Print a number to the trace log on XRPLd. Any XRPLd instance set to \"trace\" log level will
    /// see this.
    ///
    /// # Parameters
    /// * `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// * `msg_read_len`: The byte length of the text to send to the trace log.
    /// * `number`: Any integer you wish to display after the text.
    ///
    /// # Returns
    ///
    /// Returns a tuple `(i32, i64)`:
    /// * `_0` (Status Code): An `i32` indicating the result of the operation.
    ///     A value of `0` signifies success. Non-zero values indicate an error
    ///     (e.g., incorrect buffer sizes).
    /// * `_1` (Bytes Written): The number of bytes written into `keylet_buf`.
    ///     On success (`status_code == 0`), this value will be `34`.
    pub fn trace_num(msg_read_ptr: u32, msg_read_len: usize, number: i64) -> i32;
}
