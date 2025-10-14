#![no_std]
#![allow(unused_imports)]
use xrpl_std::host;
extern crate alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use blake3::Hasher;
#[unsafe(no_mangle)]
pub fn finish() -> i32 {
    // Create a 1KB (1024 bytes) array
    let data = [0x42u8; 1024];
    
    // Hash the 1KB array with blake3
    let mut hasher = Hasher::new();
    hasher.update(&data);
    let hash = hasher.finalize();

    for b in hash.as_bytes() {
        if *b > 0 {
            return 1;
        }
    }

    0
}
