use crate::host;

const MAX_LOG_LEN: usize = 256;

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
    // if message_bytes.len() > MAX_LOG_LEN {
    //     let warning = "[Warning: Log message truncated]";
    //     unsafe { log_message(warning.as_ptr(), warning.len()); }
    // }
}
