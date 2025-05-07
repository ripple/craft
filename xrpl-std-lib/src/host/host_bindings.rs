const NEXT_AVAIALBE_SLOT_NUM: u8 = 0u8;
const CUR_TX_SLOT_NUM: u8 = 201u8;
const WASM_CONTEXT_SLOT_NUM: u8 = 202u8;
const CUR_LEDGER_INFO_SLOT_NUM: u8 = 203u8;

// Defines the host functions advertised by the xrpld host.
// #[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "host")]
unsafe extern "C" {

    // #############################
    // Definitions
    // #############################

    // * `Slot`: Reserved memory in the host (sort of like a register) that WASM developers can
    // store ledger data into. Slot numbers are 1 byte (u8), which means valid values are between
    // `0` and `255`.
    //  -- Slot `0` is reserved to indicate the "next available slot";
    //  -- Slot `1 - 200` is available to the user;
    //  -- Slot `201` is reserved for `CURRENT_TRANSACTION`;
    //  -- Slot 202 is reserved for the `WASM_CONTEXT` (i.e., the ledger object that the WASM is
    // attached to).
    //  -- Slot 203 is reserved for current ledger data.
    // -- All remaining slots up to 255 are reserved.
    // RULE: Can only slot a full ledger-object; to get at a field, must use a locator
    // *Slotted Data*: Any data in rippled that can be placed into one of 255 available "slots" in
    //  rippled memory. Slots allow any particular data to be easily be referenced from WASM code.
    //  Currently, slots can hold
    // *Locator*: A collection of bytes that can be parsed to located a particular field in a ledger
    //  objects, transaction, or otherwise.

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
    pub fn trace(msg_read_ptr: u32, msg_read_len: usize, data_read_ptr: u32, data_read_len: usize, as_hex: u32) -> i64;

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
    pub fn trace_num(msg_read_ptr: u32, msg_read_len: usize, number: i64) -> i64;

    // [LEDGER] [YES] Data Access Group #4: Ledger Object (by keylet)
    // [CURRENT_TRANSACTION/TX/TRIGGER] [YES/FIXED SLOT] Data Access Group #1: Originating TX --> Not needed, see `otxn` Category.
    // [CONTEXT] [YES/FIXED SLOT] Data Access Group #2: Ledger Object with _this_ WASM contract --> See `context_keylet`
    // [LEDGER_INFO] [NO] Data Access Group #3: Info about current ledger --> Not needed, see `utils`

    // ####################################
    // Host Function Category: SLOTTED DATA
    // (Transactions + Ledger Objects)
    // ####################################

    /// Read a field from a slotted object using a locator.
    ///
    /// # Parameters
    ///
    /// * `locator_ptr`: A pointer to a 34-byte array representing a Keylet.
    /// * `locator_len`: An index representing a slot number. Valid values are between 1 and 256.
    /// * `output_ptr`: A pointer to a fixed-size array (based on the locator) where the bytes of
    ///     the field being read will be written to.
    /// * `output_len`: The expected length of the field to be read.
    /// # Returns
    ///
    /// Returns a tuple `(i32, i64)`:
    /// * `_0` (Status Code): An `i32` indicating the result of the operation.
    ///     A value of `0` signifies success. Non-zero values indicate an error
    ///     (e.g., incorrect buffer sizes).
    /// * `_1` (Slot Number): The slot number where the value represented by the Keylet is stored.
    ///
    /// # Errors
    ///
    /// TODO: Define Error Codes.
    ///
    /// # Examples
    ///
    /// ```
    /// use xrpl_std_lib::core::types::keylets::AccountKeylet;
    /// use xrpl_std_lib::core::types::locator::Locator;
    /// use xrpl_std_lib::host::{slot_set_by_num, slot_read_field};
    /// use xrpl_std_lib::host::trace::{trace_msg, trace_num};
    ///
    /// // 1. Get a Keylet to a SignerList
    /// let signer_list_keylet_buf: [u8;34] = [0u8;34];
    /// // 2. Slot the SignerList (by Keyley) and get slot_num
    /// let signer_list_slot:i64 = unsafe {
    ///     slot_set_by_num(signer_list_keylet_buf.as_ptr() as i32, 1i32).1
    /// };
    ///
    /// let slot_num_u8:u8 = if signer_list_slot >= 0 && signer_list_slot < 255 {
    ///     signer_list_slot as u8
    /// }else {
    ///     // ERR
    ///     return // TODO: What is this?
    /// };
    ///
    /// let sfSignerEntries = 1; // TODO: Make constant
    /// let sfSignerWeight = 2; // TODO: Make Constant
    /// // TODO: Add error handling to example
    /// // 3. Get a Locator to the `SignerWeight` field of the first signer in the SignerList in
    /// //    `slot_num`
    /// let mut locator:Locator = Locator::new(slot_num_u8);
    /// if locator.pack_sfield(sfSignerEntries){ // Points to SignerEntries
    ///   trace_msg("Packed SignerEntries");
    /// }
    ///
    /// if locator.pack_array_index(1){ // Points to SignerEntry #1
    ///    trace_msg("Packed Index #1");
    /// }
    /// if locator.pack_sfield(sfSignerWeight){ // Points to SignerWeight
    ///   trace_msg("Packed SignerWeight");
    /// }
    ///
    /// // TODO: Handle error code
    /// let signer_weight = unsafe {
    ///     // TODO: Sugar function: slot_read(&locator);
    ///     slot_read_field(locator.get_addr() as i32, locator.num_packed_bytes() as i32, 0i32, 0i32).1
    /// };
    /// ```
    pub fn slot_read_field(locator_ptr: i32, locator_len: i32, output_ptr: i32, output_len: i32) -> (i32, i64);

    // Mayukha's WASM allocation style
    // return two values: error_code & pointer to the result.
    // pub fn get_field(locator_ptr: i32, locator_len: i32) -> (i32, *const u8/i64);
    // TODO: Can we allocate only e.g., 8 bytes for an amount (or do we need a whole page each time)
    
    // ##############################
    // Host Function Category: LEDGER
    // ##############################

    /// Locate an object based on its keylet and place it into the specified slot number.
    ///
    /// # Parameters
    ///
    /// * `keylet_ptr`: A pointer to a 34-byte array representing a Keylet.
    /// * `slot_num`: An index representing a slot number. Valid values are between 1 and 256.
    ///
    /// # Returns
    ///
    /// Returns a tuple `(i32, i64)`:
    /// * `_0` (Status Code): An `i32` indicating the result of the operation.
    ///     A value of `0` signifies success. Non-zero values indicate an error
    ///     (e.g., incorrect buffer sizes).
    /// * `_1` (Slot Number): The slot number where the value represented by the Keylet is stored.
    ///
    /// # Errors
    ///
    /// TODO: Define Error Codes.
    ///
    /// # Examples
    ///
    /// ```
    /// use xrpl_std_lib::core::constants::ACCOUNT_ONE;
    /// use xrpl_std_lib::core::types::account_id::AccountID;
    /// use xrpl_std_lib::host::{slot_set_by_num};
    /// use xrpl_std_lib::utils::keylet::{account_keylet, Keylet};
    ///
    /// let account_id:AccountID = ACCOUNT_ONE;
    /// let account_keylet: Keylet = account_keylet(account_id);
    /// let account_slot:i64 = unsafe {
    ///     slot_set_by_num(account_keylet.0.as_ptr() as i32, 1).1
    /// };
    /// ```
    pub fn ledger_slot_set(keylet_ptr: i32, slot_num: i32) -> (i32, i64);
    pub fn slot_set(locator_ptr: i32, slot_num: i32) -> (i32, i64);
    
    // #######################################
    // Host Function Category: CURRENT_TRANSACTION
    // #######################################

    // No host functions needed because the current transaction that triggered the executing of 
    // any WASM is always available in a predefined, reserved slot number.
    
    // ###########################################
    // Host Function Category: WASM_CONTEXT
    // ###########################################
    
    // No host functions needed because the WASM context ledger object is always available in a 
    // predefined, reserved slot number.

    // #############################
    // Host Function Category: KEYLET UTILS
    // #############################

    // TODO: The hooks Keylet API is somewhat confusing with many options. Instead, it would be
    // nice if we had a dedicate host function for each keylet type. However, creating host
    // functions for every KeyLet type is too many host functions. But, since a Keylet can be
    // created using the Sha512-half and some know constants. Therefore, consider not having any
    // keylet host functions, and just implement a function for each keylet  type in the
    // "sugar" layer.

    // #############################
    // Host Function Category: UTILS
    // #############################

    /// Compute a Sha512 hash over the data pointed to by read_ptr. Write the first half of the
    /// hash to the buffer pointed to by `write_ptr`.
    pub fn sha_512_half(write_ptr: i32, write_len: i32, read_ptr: i32, read_len: i32) -> (i32, i64);

    /// Retrieve the 32-byte Sha512-Half has of the last closed ledger.
    /// `write_prt`: A pointer to a byte array of length 32.
    pub fn get_ledger_hash(write_ptr: i32) -> (i32, i64);

    /// Get the sequence number of the last ledger.
    /// Return the number in the i64.
    pub fn get_ledger_seq() -> (i32, i64);

    /// Get the close time of the last ledger.
    /// Return the XRPL timestamp of the last closed ledger in the i64 return value.
    pub fn get_ledger_close_time() -> (i32, i64);

    /// Check if a given amendment is enabled.
    /// param: `write_ptr`: A pointer to a byte array of length 256.
    /// Indicate true=1; false=0
    pub fn amendment_enabled(write_ptr: i32) -> (i32, i64);

    /// Get the current transaction base fee.
    /// Return the base fee as the i64 return value
    pub fn get_base_fee() -> (i32, i64);

    // #############################
    // Future Host Functions
    // #############################

    // 1. nonce()
    // 2. check_sig(Blob signature, Blob pubkey)?
    // 3. NFT?
    // 4. pub fn trace_float(mread_ptr: u32, mread_len: u32, float1: i64) -> i64;
    // 5. otxn: `id`, `type` (get type of otxn, currently it's always a SmartEscrow).
    // TBD.

    // #############################
    // DEPRECATED Host Functions
    // #############################

    /// Get the transaction id of the EscrowFinish transaction that instigated a Smart Escrow
    /// WASM execution.
    /// [Deprecated] Will be removed once real Host functions are defined.
    pub fn get_tx_hash(arr_ptr: *const u8);

    /// This function allows a caller to obtain the contents of a field in the current
    /// `EscrowFinish` transaction that triggered execution of the "current" WASM contract.
    ///
    /// dst_ptr: A pointer to an array that was allocated in WASM.
    /// dst_len: The length of the array pointed to by `dst_ptr`.
    /// field_code: The sfield code for the field to obtain.
    /// [Deprecated] Will be removed once real Host functions are defined.
    pub fn get_current_escrow_finish_field(dst_ptr: *const u8, dst_len: usize, field_code: i32) -> i64;
}

// #############################
// SUGAR Layer
// #############################

// Functions for each Keylet.
// pub fn account_keylet(account_id:AccountID);
// ... --> 26
// Structs for various types of object (e.g., Hash256, AccountID, etc).
// Functions for packing and unpacking a Locator (if we keep locators).

// 1. Advantages of Locator
// 1. Uses fewer slots to find a field
// 2. Reduces # of host functions to deal with slots (i.e., no `slot_subarray`; no `slot_subfield`)
// 2. Disadvantages of Locator
// 1. Any locator is always 64-bytes, so these bytes get sent across the WASM boundary on every call.
// 2. Mental overhead of dealing with slots + locator concepts vs. only slots
// 3. # of lines to read one field is slightly more (1 line more) than hooks slot API.

// #############################
// Testing this ABI Rust
// #############################

// TODO: For testing purposes, uncomment the cfg directive above and below, and implement Rust
// variants of the host functions. This would be for testing purposes only.
// #[cfg(not(target_arch = "wasm32"))]
// pub fn trace(
//     _msg_read_ptr: u32,
//     _msg_read_len: usize,
//     _data_read_ptr: u32,
//     _data_read_len: usize,
//     _as_hex: u32,
// ) -> i64 {
//     return -1;
// }
// ... and other functions
