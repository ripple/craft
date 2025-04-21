use crate::core::types::Hash256;
use crate::host;

/// Helper to log a `&str` to the host's standard out.
pub fn log(message: &str) {
    log_with_newline(message, false)
}

/// Helper to log a `&str` (with an optional newline) to the host's standard out.
pub fn log_with_newline(message: &str, with_newline: bool) {
    if with_newline {
        unsafe { host::log_ln(message.as_ptr(), message.len()) }
    } else {
        unsafe { host::log(message.as_ptr(), message.len()) }
    }
}

/// Log a byte array of size `len` as a hex string.
pub fn log_hash_ref(prefix: &str, hash: &Hash256) {
    unsafe {
        host::log(prefix.as_ptr(), prefix.len());
        host::log_hex(hash.0.as_ptr(), 32);
    }
}
