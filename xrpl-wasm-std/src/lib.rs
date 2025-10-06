#![doc = include_str!("../README.md")]
#![no_std]

pub mod core;
pub mod host;
pub mod sfield;
pub mod types;

/// Additional guides and how-tos
#[cfg(doc)]
pub mod guides {
    /// XRPL Field Access and Locators guide
    #[doc = include_str!("../../docs/FIELD_ACCESS.md")]
    pub mod field_access {}

    /// XRPL Float Operations (IOU format and math)
    #[doc = include_str!("../../docs/FLOAT_OPERATIONS.md")]
    pub mod float_operations {}
}

/// This function is called on panic but only in the WASM architecture. In non-WASM (e.g., in the
/// Host Simulator) the standard lib is available, which includes a panic handler.
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(info: &::core::panic::PanicInfo) -> ! {
    // Emit panic information to the trace log before halting execution
    // This ensures that panic messages are visible in the terminal/logs

    // Try to emit the panic message using trace functions
    // We use unsafe operations here because we're in a panic handler
    // and need to be very careful about what we do

    // First, emit a clear panic indicator
    let panic_msg = "WASM PANIC OCCURRED";
    unsafe {
        crate::host::trace(
            panic_msg.as_ptr(),
            panic_msg.len(),
            ::core::ptr::null(),
            0,
            0, // as UTF-8
        )
    };

    // Note: info.payload() is deprecated and doesn't provide useful information
    // We rely on the panic message from the panic! macro instead

    // Try to emit location information if available
    if let Some(location) = info.location() {
        let location_msg = "Panic location: ";
        let file_name = location.file();
        unsafe {
            crate::host::trace(
                location_msg.as_ptr(),
                location_msg.len(),
                file_name.as_ptr(),
                file_name.len(),
                0, // as UTF-8
            )
        };

        // Emit line number separately since we can't format strings in no_std
        let line_msg = "Line number: ";
        let _ =
            unsafe { host::trace_num(line_msg.as_ptr(), line_msg.len(), location.line() as i64) };
    }

    // This instruction will halt execution of the WASM module.
    // It's the WASM equivalent of a trap or an unrecoverable error.
    ::core::arch::wasm32::unreachable();
}

fn hex_char_to_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// Decode a 64-hex-character string into a 32-byte array.
///
/// The input must be exactly 64 hexadecimal ASCII bytes (lower- or upper-case).
/// Returns `None` if any character is not a valid hex digit.
///
/// Example:
/// ```
/// # use xrpl_wasm_std::decode_hex_32;
/// let hex = *b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
/// let bytes = decode_hex_32(&hex).unwrap();
/// assert_eq!(bytes.len(), 32);
/// ```
pub fn decode_hex_32(hex: &[u8; 64]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let high = hex_char_to_nibble(hex[i * 2])?;
        let low = hex_char_to_nibble(hex[i * 2 + 1])?;
        out[i] = (high << 4) | low;
        i += 1;
    }
    Some(out)
}

/// Decode a 40-hex-character string into a 20-byte array.
///
/// The input must be exactly 40 hexadecimal ASCII bytes.
/// Returns `None` if any character is not a valid hex digit.
///
/// Example:
/// ```
/// # use xrpl_wasm_std::decode_hex_20;
/// let hex = *b"00112233445566778899aabbccddeeff00112233";
/// let bytes = decode_hex_20(&hex).unwrap();
/// assert_eq!(bytes.len(), 20);
/// ```
pub fn decode_hex_20(hex: &[u8; 40]) -> Option<[u8; 20]> {
    let mut out = [0u8; 20];
    let mut i = 0;
    while i < 20 {
        let high = hex_char_to_nibble(hex[i * 2])?;
        let low = hex_char_to_nibble(hex[i * 2 + 1])?;
        out[i] = (high << 4) | low;
        i += 1;
    }
    Some(out)
}
