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
    /// - `out_buff_ptr`: A mutable raw pointer to the buffer where the ledger sequence number will
    ///   be written.
    /// - `out_buff_len`: Specifies the size of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves the parent ledger time.
    ///
    /// This function is used to obtain the parent ledger's timestamp as a byte array.
    /// The timestamp is written into a provided output buffer.
    ///
    /// # Parameters
    /// - `out_buff_ptr`: A mutable pointer to the output buffer where the parent ledger time will
    ///   be stored. The buffer should be pre-allocated with enough space to hold the data.
    /// - `out_buff_len`: The length of the output buffer. This value must be at least as large as
    ///   the data intended to be written to avoid memory issues.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves the hash of the parent ledger.
    ///
    /// This function fetches the hash of the parent ledger and stores it in the buffer provided.
    /// The hash is expected to be written to the memory location pointed by `out_buff_ptr`,
    /// and its length should not exceed the `out_buff_len`.
    ///
    /// # Parameters
    /// - `out_buff_ptr`: A mutable pointer to a buffer where the parent ledger hash will be written.
    ///   The buffer must be allocated and managed by the caller.
    /// - `out_buff_len`: The maximum length of the buffer in bytes. This indicates the size of the
    ///   buffer and ensures that the function does not write beyond the allowed length.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
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
    /// - `keylet_ptr`: A raw pointer to the keylet, which is a unique identifier used to locate or
    ///   store data in the ledger.
    /// - `keylet_len`: The length of the keylet specified by `keylet_ptr`.
    /// - `cache_num`: The cache number to which the keylet will be placed in. If 0, the host will
    ///   assign a new cache space.
    ///
    /// # Returns
    ///
    /// - Returns a positive cache number
    /// - Returns a negative error code on failure
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
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
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves a specific transaction field with two field identifiers and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - The primary field identifier.
    /// * `field2` - The secondary field identifier.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_tx_field2(
        field: i32,
        field2: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific transaction field with three field identifiers and writes it into the
    /// provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - The primary field identifier.
    /// * `field2` - The secondary field identifier.
    /// * `field3` - The tertiary field identifier.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_tx_field3(
        field: i32,
        field2: i32,
        field3: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific transaction field with four field identifiers and writes it into the
    /// provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - The primary field identifier.
    /// * `field2` - The secondary field identifier.
    /// * `field3` - The tertiary field identifier.
    /// * `field4` - The quaternary field identifier.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_tx_field4(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific transaction field with five field identifiers and writes it into the
    /// provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - The primary field identifier.
    /// * `field2` - The secondary field identifier.
    /// * `field3` - The tertiary field identifier.
    /// * `field4` - The quaternary field identifier.
    /// * `field5` - The quinary field identifier.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_tx_field5(
        field: i32,
        field2: i32,
        field3: i32,
        field4: i32,
        field5: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific transaction field with six field identifiers and writes it into the
    /// provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - The primary field identifier.
    /// * `field2` - The secondary field identifier.
    /// * `field3` - The tertiary field identifier.
    /// * `field4` - The quaternary field identifier.
    /// * `field5` - The quinary field identifier.
    /// * `field6` - The senary field identifier.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    #[allow(clippy::too_many_arguments)]
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

    /// Retrieves a specific field from the current ledger object and writes it into the provided
    /// output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - An integer value representing the specific field to retrieve from the current ledger object.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_current_ledger_obj_field(
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific field from a cached ledger object and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `cache_num` - The cache number identifying the specific ledger object.
    /// * `field` - An integer value representing the specific field to retrieve from the ledger object.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with an unallocated buffer or a length that exceeds the
    /// specified buffer's allocated memory.
    unsafe fn get_ledger_obj_field(
        cache_num: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from the transaction using a field locator and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `locator_ptr` - A pointer to a buffer containing the field locator that identifies the nested field.
    /// * `locator_len` - The length of the field locator buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with unallocated buffers or lengths that exceed the
    /// specified buffers' allocated memory.
    unsafe fn get_tx_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from the current ledger object using a field locator and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `locator_ptr` - A pointer to a buffer containing the field locator that identifies the nested field.
    /// * `locator_len` - The length of the field locator buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with unallocated buffers or lengths that exceed the
    /// specified buffers' allocated memory.
    unsafe fn get_current_ledger_obj_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from a cached ledger object using a field locator and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `cache_num` - The cache number identifying the specific ledger object.
    /// * `locator_ptr` - A pointer to a buffer containing the field locator that identifies the nested field.
    /// * `locator_len` - The length of the field locator buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function should not be called with unallocated buffers or lengths that exceed the
    /// specified buffers' allocated memory.
    unsafe fn get_ledger_obj_nested_field(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the length of an array field in the transaction.
    ///
    /// # Parameters
    ///
    /// * `field` - An integer value representing the specific array field in the transaction.
    ///
    /// # Returns
    ///
    /// - Returns a positive number representing the length of the array on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses transaction data through raw memory operations.
    unsafe fn get_tx_array_len(field: i32) -> i32;

    /// Retrieves the length of an array field in the current ledger object.
    ///
    /// # Parameters
    ///
    /// * `field` - An integer value representing the specific array field in the current ledger object.
    ///
    /// # Returns
    ///
    /// - Returns a positive number representing the length of the array on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses ledger object data through raw memory operations.
    unsafe fn get_current_ledger_obj_array_len(field: i32) -> i32;

    /// Retrieves the length of an array field in a cached ledger object.
    ///
    /// # Parameters
    ///
    /// * `cache_num` - The cache number identifying the specific ledger object.
    /// * `field` - An integer value representing the specific array field in the ledger object.
    ///
    /// # Returns
    ///
    /// - Returns a positive number representing the length of the array on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses ledger object data through raw memory operations.
    unsafe fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32;

    /// Retrieves the length of a nested array field in the transaction using a field locator.
    ///
    /// # Parameters
    ///
    /// * `locator_ptr` - A pointer to a buffer containing the field locator that identifies the nested array field.
    /// * `locator_len` - The length of the field locator buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number representing the length of the array on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses transaction data through raw memory operations
    /// and requires a properly formatted field locator buffer.
    unsafe fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;

    /// Retrieves the length of a nested array field in the current ledger object using a field locator.
    ///
    /// # Parameters
    ///
    /// * `locator_ptr` - A pointer to a buffer containing the field locator that identifies the nested array field.
    /// * `locator_len` - The length of the field locator buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number representing the length of the array on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses ledger object data through raw memory operations
    /// and requires a properly formatted field locator buffer.
    unsafe fn get_current_ledger_obj_nested_array_len(
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    /// Retrieves the length of a nested array field in a cached ledger object using a field locator.
    ///
    /// # Parameters
    ///
    /// * `cache_num` - The cache number identifying the specific ledger object.
    /// * `locator_ptr` - A pointer to a buffer containing the field locator that identifies the nested array field.
    /// * `locator_len` - The length of the field locator buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number representing the length of the array on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses ledger object data through raw memory operations
    /// and requires a properly formatted field locator buffer.
    unsafe fn get_ledger_obj_nested_array_len(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    /// Updates data in the host environment.
    ///
    /// # Parameters
    ///
    /// * `data_ptr` - A pointer to a buffer containing the data to update.
    /// * `data_len` - The length of the data buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive value on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses and modifies data through raw memory operations.
    /// The caller must ensure that the data buffer is valid and properly formatted.
    unsafe fn update_data(data_ptr: *const u8, data_len: usize) -> i32;

    /// Computes the SHA-512 half hash of the provided data.
    ///
    /// This function calculates the SHA-512 hash of the input data and returns the first half (256 bits)
    /// of the resulting hash.
    ///
    /// # Parameters
    ///
    /// * `data_ptr` - A pointer to a buffer containing the data to hash.
    /// * `data_len` - The length of the data buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the hash will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that both the input and output buffers are valid and properly sized.
    unsafe fn compute_sha512_half(
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet for an account.
    ///
    /// A keylet is a unique identifier used to locate or store data in the ledger.
    /// This function creates a keylet for a specific account.
    ///
    /// # Parameters
    ///
    /// * `account_ptr` - A pointer to a buffer containing the account identifier.
    /// * `account_len` - The length of the account buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the keylet will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that both the input and output buffers are valid and properly sized.
    unsafe fn account_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet for a credential.
    ///
    /// A keylet is a unique identifier used to locate or store data in the ledger.
    /// This function creates a keylet for a specific credential based on subject, issuer, and credential type.
    ///
    /// # Parameters
    ///
    /// * `subject_ptr` - A pointer to a buffer containing the subject identifier.
    /// * `subject_len` - The length of the subject buffer.
    /// * `issuer_ptr` - A pointer to a buffer containing the issuer identifier.
    /// * `issuer_len` - The length of the issuer buffer.
    /// * `cred_type_ptr` - A pointer to a buffer containing the credential type.
    /// * `cred_type_len` - The length of the credential type buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the keylet will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that all input and output buffers are valid and properly sized.
    #[allow(clippy::too_many_arguments)]
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

    /// Generates a keylet for an escrow.
    ///
    /// A keylet is a unique identifier used to locate or store data in the ledger.
    /// This function creates a keylet for a specific escrow based on account and sequence number.
    ///
    /// # Parameters
    ///
    /// * `account_ptr` - A pointer to a buffer containing the account identifier.
    /// * `account_len` - The length of the account buffer.
    /// * `sequence` - The sequence number of the escrow.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the keylet will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that both the input and output buffers are valid and properly sized.
    unsafe fn escrow_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet for an oracle.
    ///
    /// A keylet is a unique identifier used to locate or store data in the ledger.
    /// This function creates a keylet for a specific oracle based on account and document ID.
    ///
    /// # Parameters
    ///
    /// * `account_ptr` - A pointer to a buffer containing the account identifier.
    /// * `account_len` - The length of the account buffer.
    /// * `document_id` - The document ID associated with the oracle.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the keylet will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that both the input and output buffers are valid and properly sized.
    unsafe fn oracle_keylet(
        account_ptr: *const u8,
        account_len: usize,
        document_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves information about a Non-Fungible Token (NFT) owned by a specific account.
    ///
    /// # Parameters
    ///
    /// * `account_ptr` - A pointer to a buffer containing the account identifier.
    /// * `account_len` - The length of the account buffer.
    /// * `nft_id_ptr` - A pointer to a buffer containing the NFT identifier.
    /// * `nft_id_len` - The length of the NFT identifier buffer.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the NFT information will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that all input and output buffers are valid and properly sized.
    unsafe fn get_nft(
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Outputs a trace message with optional data for debugging purposes.
    ///
    /// # Parameters
    ///
    /// * `msg_read_ptr` - A pointer to a buffer containing the message to trace.
    /// * `msg_read_len` - The length of the message buffer.
    /// * `data_read_ptr` - A pointer to a buffer containing the data to trace.
    /// * `data_read_len` - The length of the data buffer.
    /// * `as_hex` - A flag indicating whether to format the data as hexadecimal (non-zero) or not (zero).
    ///
    /// # Returns
    ///
    /// - Returns a positive value on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that the message and data buffers are valid and properly sized.
    unsafe fn trace(
        msg_read_ptr: u32,
        msg_read_len: usize,
        data_read_ptr: u32,
        data_read_len: usize,
        as_hex: u32,
    ) -> i32;

    /// Outputs a trace message with a numeric value for debugging purposes.
    ///
    /// # Parameters
    ///
    /// * `msg_read_ptr` - A pointer to a buffer containing the message to trace.
    /// * `msg_read_len` - The length of the message buffer.
    /// * `number` - The numeric value to trace.
    ///
    /// # Returns
    ///
    /// - Returns a positive value on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that the message buffer is valid and properly sized.
    unsafe fn trace_num(msg_read_ptr: u32, msg_read_len: usize, number: i64) -> i32;

    /// Outputs a trace message with an opaque floating-point value for debugging purposes.
    ///
    /// # Parameters
    ///
    /// * `msg_read_ptr` - A pointer to a buffer containing the message to trace.
    /// * `msg_read_len` - The length of the message buffer.
    /// * `opaque_float_ptr` - A pointer to an opaque floating-point value to trace.
    ///
    /// # Returns
    ///
    /// - Returns a positive value on success
    /// - Returns a negative error code on failure.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it accesses data through raw memory operations.
    /// The caller must ensure that the message buffer and opaque float pointer are valid and properly sized.
    unsafe fn trace_opaque_float(
        msg_read_ptr: u32,
        msg_read_len: usize,
        opaque_float_ptr: u32,
    ) -> i32;
}
