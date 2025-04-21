use crate::core::types::Hash256;
use crate::host;

pub fn get_tx_id() -> Hash256 {
    // 1. Allocate a buffer on the stack
    let buffer = [0u8; 32]; // Allocate memory to read into.

    // 2. Call the actual host function with a pointer to the above array.
    unsafe {
        // Pass pointer to the start of our stack buffer (and the number of bytes copied) to the
        // host function for proper logging.
        host::get_tx_hash(buffer.as_ptr());
    }

    // 3. Return the transactionId as a Hash256.
    buffer.into()
}
