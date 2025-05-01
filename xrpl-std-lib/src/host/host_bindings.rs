// Defines the host functions advertised by the xrpld host.
// #[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "host")]
unsafe extern "C" {

    #[doc = " Print to the trace log on xrpld. Any xrpld instance set to \"trace\" log level will see this."]
    #[doc = " @param msg_read_ptr A buffer containing either text (in either utf8, or utf16le)"]
    #[doc = " @param msg_read_len The byte length of the text to send to the trace log"]
    #[doc = " @param data_read_ptr A buffer containing data (in either utf8, or utf16le)"]
    #[doc = " @param data_read_len The byte length of the data to send to the trace log"]
    #[doc = " @param as_hex If 0 treat the data_read_ptr as pointing at a string of text, otherwise treat it as data and print hex"]
    #[doc = " @return The number of bytes output or a negative integer if an error occurred."]
    pub fn trace(msg_read_ptr: u32, msg_read_len: usize, data_read_ptr: u32, data_read_len: usize, as_hex: u32) -> i64;

    #[doc = " Print to the trace log on xrpld along with a decimal number. Any xrpld instance set to \"trace\" log level will see this."]
    #[doc = " @param read_ptr A pointer to the string to output"]
    #[doc = " @param read_len The length of the string to output"]
    #[doc = " @param number Any integer you wish to display after the text"]
    #[doc = " @return A negative value on error"]
    pub fn trace_num(read_ptr: u32, read_len: u32, number: i64) -> i64;

    // TODO: Implement this once floats are worked out.
    //pub fn trace_float(mread_ptr: u32, mread_len: u32, float1: i64) -> i64;

    /// Log a byte array of size `len` as a UTF-8 string.
    // pub fn log(str_ptr: *const u8, len: usize);

    /// Log a byte array of size `len` as a UTF-8 string (with a trailing newline).
    // pub fn log_ln(str_ptr: *const u8, len: usize);

    /// Log a byte array of size `len` as a hex string.
    // pub fn log_hex(byte_ptr: *const u8, len: usize);

    /// Get the transaction id of the EscrowFinish transaction that instigated a Smart Escrow
    /// WASM execution.
    pub fn get_tx_hash(arr_ptr: *const u8);

    /// This function allows a caller to obtain the contents of a field in the current
    /// `EscrowFinish` transaction that triggered execution of the "current" WASM contract.
    ///
    /// dst_ptr: A pointer to an array that was allocated in WASM.
    /// dst_len: The length of the array pointed to by `dst_ptr`.
    /// field_code: The sfield code for the field to obtain.
    pub fn get_current_escrow_finish_field(dst_ptr: *const u8, dst_len: usize, field_code: i32) -> i64;
}

// TODO: For testing purposes, uncomment the cfg directive above and below, and implement Rust
// variants of the host functions. This would be for testing purposes only.
// #[cfg(not(target_arch = "wasm32"))]
