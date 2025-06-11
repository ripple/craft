#![no_std]

pub mod core;
pub mod host;
pub mod locator;
pub mod sfield;
pub mod types;

/// This function is called on panic, but only in the WASM architecture. In non-WASM (e.g., in the
/// Host Simulator) the standard lib is available, which includes a panic handler.
#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &::core::panic::PanicInfo) -> ! {
    // This instruction will halt execution of the WASM module.
    // It's the WASM equivalent of a trap or an unrecoverable error.
    ::core::arch::wasm32::unreachable();
}
