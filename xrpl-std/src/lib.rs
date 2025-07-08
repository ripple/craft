#![no_std]

pub mod core;
pub mod host;
pub mod sfield;
pub mod types;

/// This function is called on panic but only in the WASM architecture. In non-WASM (e.g., in the
/// Host Simulator) the standard lib is available, which includes a panic handler.
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
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
