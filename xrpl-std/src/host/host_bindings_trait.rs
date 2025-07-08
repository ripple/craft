/// This trait defines the interface for host bindings.
/// 
/// It provides a common interface for both WASM and testing implementations
/// of the host functions.
pub trait HostBindings {
    /// Retrieves the current ledger sequence number.
    ///
    /// This function populates a provided buffer with the ledger sequence number.
    ///
    /// # Arguments
    ///
    /// - `out_buff_ptr`: A mutable raw pointer to the buffer where the ledger sequence
    ///                   number will be written.
    /// - `out_buff_len`: Specifies the size of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    unsafe fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves the parent ledger time.
    ///
    /// This function is used to obtain the parent ledger's timestamp as a byte array.
    /// The timestamp is written into a provided output buffer.
    ///
    /// # Parameters
    /// - `out_buff_ptr`: A mutable pointer to the output buffer where the parent ledger time will
    ///                   be stored. The buffer should be pre-allocated with enough space to hold
    ///                   the data.
    /// - `out_buff_len`: The length of the output buffer. This value must be at least as large as
    ///                   the data intended to be written to avoid memory issues.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    unsafe fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves the hash of the parent ledger.
    ///
    /// This function fetches the hash of the parent ledger and stores it in the buffer provided.
    /// The hash is expected to be written to the memory location pointed by `out_buff_ptr`,
    /// and its length should not exceed the `out_buff_len`.
    ///
    /// # Parameters
    /// - `out_buff_ptr`: A mutable pointer to a buffer where the parent ledger hash will be written.
    ///                   The buffer must be allocated and managed by the caller.
    /// - `out_buff_len`: The maximum length of the buffer in bytes. This indicates the size of the
    ///                   buffer and ensures that the function does not write beyond the allowed
    ///                   length.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    unsafe fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Fetch a ledger entry pointed by the given keylet.
    ///
    /// This function uses the keylet to locate a ledger entry. If found, add it to the
    /// cache. The cache can have up to 255 ledger entries. If `cache_num` is 0, the
    /// new ledger entry will put in the next available cache space. If `cache_num` is not 0,
    /// the new ledger entry will replace an existing ledger entry in the catch.
    ///
    /// # Parameters
    ///
    /// - `keylet_ptr`: A raw pointer to the keylet, which is a unique identifier used to
    ///                 locate or store data in the ledger.
    /// - `keylet_len`: The length of the keylet specified by `keylet_ptr`.
    /// - `cache_num`: The cache number to which the keylet will be placed in.
    ///                If 0, the host will assign a new cache space.
    ///
    /// # Returns
    ///
    /// - Returns a positive cache number
    /// - Returns a negative error code on failure
    unsafe fn cache_ledger_obj(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32;

    /// Retrieves a specific transaction field and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - An integer value representing the specific transaction field to retrieve.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    unsafe fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    
    unsafe fn get_tx_field2(
        field: i32,
        field2: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    
    unsafe fn get_tx_field3(
        field: i32,
        field2: i32,
        field3: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    
    unsafe fn get_tx_field4(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    
    unsafe fn get_tx_field5(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        field5: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    
    unsafe fn get_tx_field6(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        field5: i32,
        field6: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_current_ledger_obj_field(
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_ledger_obj_field(
        cache_num: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_tx_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_current_ledger_obj_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_ledger_obj_nested_field(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_tx_array_len(field: i32) -> i32;

    unsafe fn get_current_ledger_obj_array_len(field: i32) -> i32;

    unsafe fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32;

    unsafe fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;

    unsafe fn get_current_ledger_obj_nested_array_len(
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    unsafe fn get_ledger_obj_nested_array_len(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    unsafe fn update_data(data_ptr: *const u8, data_len: usize) -> i32;

    unsafe fn compute_sha512_half(
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn account_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn credential_keylet(
        subject_ptr: *const u8,
        subject_len: usize,
        issuer_ptr: *const u8,
        issuer_len: usize,
        cred_type_ptr: *const u8,
        cred_type_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn escrow_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn oracle_keylet(
        account_ptr: *const u8,
        account_len: usize,
        document_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn get_nft(
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    unsafe fn trace(
        msg_read_ptr: u32,
        msg_read_len: usize,
        data_read_ptr: u32,
        data_read_len: usize,
        as_hex: u32,
    ) -> i32;

    unsafe fn trace_num(msg_read_ptr: u32, msg_read_len: usize, number: i64) -> i32;

    unsafe fn trace_opaque_float(msg_read_ptr: u32, msg_read_len: usize, opaque_float_ptr: u32) -> i32;
}