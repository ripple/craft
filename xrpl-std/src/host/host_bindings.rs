#[link(wasm_import_module = "host_lib")]
unsafe extern "C" {
    // ###############################
    // Host Function Category: getters
    // ###############################

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
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

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
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

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
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

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
    pub fn cache_ledger_obj(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32;

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
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
    pub fn get_tx_field2(
        field: i32,
        field2: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_tx_field3(
        field: i32,
        field2: i32,
        field3: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_tx_field4(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_tx_field5(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        field5: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;
    pub fn get_tx_field6(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        field5: i32,
        field6: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific field from the current ledger object and writes it into the provided buffer.
    ///
    /// # Parameters
    /// - `field` (`i32`): The integer identifier for the desired field in the ledger object.
    /// - `out_buff_ptr` (`*mut u8`): A mutable pointer to the memory location where the field data
    ///   will be written. This should point to a pre-allocated buffer.
    /// - `out_buff_len` (`usize`): The size (in bytes) of the buffer provided by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_current_ledger_obj_field(
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific field from a ledger object based on the given parameters.
    ///
    /// # Parameters
    ///
    /// - `cache_num`: An integer representing the cache index of the ledger object.
    /// - `field`: An integer representing the specific field to retrieve from the ledger object.
    /// - `out_buff_ptr`: A mutable pointer to a buffer where the retrieved field data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_obj_field(
        cache_num: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from the current ledger object and writes it into the provided buffer.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    /// - `out_buff_ptr`: A pointer to a mutable byte array where the resulting field data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_tx_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific nested field from the current ledger object.
    ///
    /// This function is designed to access a nested field within the ledger object
    /// specified by the `locator`. The `locator` acts as a path or identifier to
    /// the desired field. The resulting data is written to the `out_buff` buffer.
    /// The function returns a status code indicating success or failure of the operation.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    /// - `out_buff_ptr`: A pointer to a mutable byte array where the resulting field data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_current_ledger_obj_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from a ledger object in a specific cache_num and writes the result into an output buffer.
    ///
    /// # Parameters
    /// - `cache_num`: The cache index of the ledger object to access.
    /// - `locator_ptr`: A pointer to the memory location containing the locator string data
    ///                  (used to identify the nested field in the ledger object).
    /// - `locator_len`: The length of the locator string.
    /// - `out_buff_ptr`: A pointer to the buffer where the retrieved nested field value will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_obj_nested_field(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the length of an array based on the provided field value.
    ///
    /// # Parameters
    /// - `field` (i32): The integer identifier for the desired field.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_tx_array_len(field: i32) -> i32;

    /// Retrieves the length of an array based on the provided field value.
    ///
    /// # Parameters
    /// - `field` (i32): The integer identifier for the desired field.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_current_ledger_obj_array_len(field: i32) -> i32;

    /// Retrieves the length of an array based on the provided cache number and field value.
    ///
    /// # Parameters
    /// - `cache_num`: The cache index of the ledger object to access.
    /// - `field` (i32): The integer identifier for the desired field.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32;

    /// Retrieves the length of an array based on the provided locator.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;

    /// Retrieves the length of an array based on the provided locator.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_current_ledger_obj_nested_array_len(
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    /// Retrieves the length of an array based on the provided locator.
    ///
    /// # Parameters
    /// - `cache_num`: The cache index of the ledger object to access.
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_obj_nested_array_len(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    // ###################################################
    // Host Function Category: update current ledger entry
    // ###################################################
    /// Updates a data field of the current ledger entry
    ///
    /// # Parameters
    ///
    /// - `data_ptr`: A pointer to the data to be written.
    /// - `data_len`: The size of the data.
    ///
    /// # Returns
    ///
    /// - 0 on success
    /// - negative for an error
    pub fn update_data(data_ptr: *const u8, data_len: usize) -> i32;

    // ###################################################
    // Host Function Category: hash and keylet computation
    // ###################################################

    /// Computes the first 32 bytes (half) of the SHA-512 hash for the given input data.
    ///
    /// # Parameters
    ///
    /// - `data_ptr`: A pointer to the input data to be hashed.
    /// - `data_len`: The length, in bytes, of the input data.
    /// - `out_buff_ptr`: A pointer to the buffer where the resulting 32-byte hash will be written.
    /// - `out_buff_len`: The length, in bytes, of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn compute_sha512_half(
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates the keylet (key identifier) for a specific account.
    ///
    /// This function is used to calculate the account keylet in a cryptographic or
    /// blockchain-based system. A keylet is typically used to identify an account or entity
    /// in a secure and deterministic way.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory of the account identifier.
    /// - `account_len`: The size (in bytes) of the data pointed to by `account_ptr`.
    /// - `out_buff_ptr`: A pointer to the memory where the generated keylet will be stored.
    /// - `out_buff_len`: The length (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn account_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet for a credential.
    ///
    /// # Parameters
    ///
    /// * `subject_ptr`: A pointer to the memory location where the subject data begins.
    /// * `subject_len`: The length of the subject data in bytes.
    /// * `issuer_ptr`: A pointer to the memory location where the issuer data begins.
    /// * `issuer_len`: The length of the issuer data in bytes.
    /// * `cred_type_ptr`: A pointer to the memory location where the credential type data begins.
    /// * `cred_type_len`: The length of the credential type data in bytes.
    /// * `out_buff_ptr`: A pointer to the buffer where the generated keylet will be written.
    /// * `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success    
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn credential_keylet(
        subject_ptr: *const u8,
        subject_len: usize,
        issuer_ptr: *const u8,
        issuer_len: usize,
        cred_type_ptr: *const u8,
        cred_type_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for an escrow entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the escrow entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success    
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn escrow_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet associated with an oracle's account and document ID.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `document_id`: An integer representing the ID of the document associated with the oracle.
    /// - `out_buff_ptr`: A pointer to a pre-allocated buffer where the resulting keylet will be
    ///   written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success    
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn oracle_keylet(
        account_ptr: *const u8,
        account_len: usize,
        document_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    // #############################
    // Host Function Category: NFT
    // #############################

    /// Retrieves the URI details of a specific NFT (Non-Fungible Token) associated with a given account.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    /// - `out_buff_ptr`: A mutable pointer to the memory location where the retrieved NFT URI
    ///   will be written.
    /// - `out_buff_len`: The maximum length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success    
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft(
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the issuer of a specific NFT (Non-Fungible Token).
    ///
    /// # Parameters
    ///
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    /// - `out_buff_ptr`: A mutable pointer to the memory location where the retrieved issuer
    ///   account will be written.
    /// - `out_buff_len`: The maximum length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success    
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft_issuer(
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
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
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
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
    pub fn trace_num(msg_read_ptr: u32, msg_read_len: usize, number: i64) -> i32;
}
