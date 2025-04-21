use crate::core::types::Hash256;
use crate::host;
use crate::utils::string::BufferWriter;
use alloc::string::ToString;
use core::fmt::Write;
use core::usize::MAX;

const MAX_LOG_LEN: usize = 256;
const HASH_256_LEN: usize = 32;

// Helper trait/function for hex formatting (replace with hex crate if available/preferred)
trait ToHex {
    fn write_hex<W: Write>(&self, writer: &mut W) -> core::fmt::Result;
}

impl ToHex for Hash256 {
    fn write_hex<W: Write>(&self, writer: &mut W) -> core::fmt::Result {
        for byte in self.0 {
            // Accessing the inner [u8; 32]
            // match
            write!(writer, "{:02X}", byte)?
            // {
            // Ok(_) => log("Wrote 1 bytes"),
            // Err(e) => {
            //     log(&e.to_string());
            //     log("Wrote 0 bytes")
            // }
            // }
        }
        Ok(())
    }
}

/// Helper to log a `&str` to the host's standard out. Note that logging emissions are capped at
/// 256 bytes.
pub fn log(message: &str) {
    // 1. Allocate a buffer on the stack
    let mut buffer = [0u8; MAX_LOG_LEN]; // Choose a reasonable fixed size

    // 2. Get the bytes of the input message
    let message_bytes = message.as_bytes();

    // 3. Determine how many bytes to copy (minimum of message length and buffer capacity)
    let len_to_copy = core::cmp::min(message_bytes.len(), MAX_LOG_LEN);

    // 4. Copy the message bytes into the buffer
    buffer[..len_to_copy].copy_from_slice(&message_bytes[..len_to_copy]);

    // 5. Call the host function with the pointer and *actual copied length* from the buffer
    unsafe {
        // Pass pointer to the start of our stack buffer (and the number of bytes copied) to the
        // host function for proper logging.
        host::log(buffer.as_ptr(), len_to_copy);
    }

    // Optional: You could add logic here to detect and report truncation
    if message_bytes.len() > MAX_LOG_LEN {
        let warning = "[Warning: Log message truncated]";
        unsafe {
            host::log(warning.as_ptr(), warning.len());
        }
    }
}
pub fn log_hash_ref(prefix: &str, hash: &Hash256) {
    let mut buffer = [0u8; MAX_LOG_LEN];
    let final_len: usize;

    {
        // Inner scope for the writer
        let mut writer = BufferWriter {
            buffer: &mut buffer,
            cursor: 0,
        };
        if write!(writer, "{}: ", prefix).is_ok() {
            if hash.write_hex(&mut writer).is_ok() {
                final_len = writer.cursor; // Store length on success
            } else {
                final_len = writer.cursor; // Store partial length on hex write failure
                // Optionally overwrite buffer with error message here if possible
            }
        } else {
            final_len = writer.cursor; // Store partial length on prefix write failure
            // Optionally overwrite buffer with error message here if possible
        }
    } // writer goes out of scope

    if final_len > 0 {
        // Only log if something was written
        unsafe {
            host::log(buffer.as_ptr(), final_len);
        }
    } else {
        // Log generic error if even prefix failed completely?
        log("Error: Log buffer too small for prefix.");
    }
}
