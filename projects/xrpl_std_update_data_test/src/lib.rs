#![no_std]

extern crate xrpl_std;

use xrpl_std::host;

// A simple struct we want to send to the host
struct TestData {
    id: u32,
    value: [u8; 4], // Some fixed-size data
}

impl TestData {
    // Simple serialization: just an example
    // For real use, consider a serialization format (like CBOR or a custom one)
    fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&self.id.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.value);
        bytes
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let msg1 = "WASM: xrpl_std_update_data_test finish() called.";
    // host::trace(msg_ptr, msg_len, data_ptr, data_len, as_hex)
    // Logging only a message, so data_ptr and data_len are 0.
    unsafe { host::trace(msg1.as_ptr() as u32, msg1.len(), 0, 0, 0); }

    // 1. Create some test data
    let test_data = TestData {
        id: 12345,
        value: [0xAA, 0xBB, 0xCC, 0xDD],
    };
    let msg2 = "WASM: Test data created.";
    unsafe { host::trace(msg2.as_ptr() as u32, msg2.len(), 0, 0, 0); }

    // 2. Serialize it
    let data_bytes = test_data.to_bytes();
    let msg3 = "WASM: Test data serialized. Data to send (hex):";
    // Log msg3 and then the data_bytes as hex
    unsafe { host::trace(msg3.as_ptr() as u32, msg3.len(), data_bytes.as_ptr() as u32, data_bytes.len(), 1); }

    // 3. Call host::update_data
    unsafe {
        host::update_data(data_bytes.as_ptr(), data_bytes.len());
    }
    let msg4 = "WASM: host::update_data called and assumed success.";
    unsafe { host::trace(msg4.as_ptr() as u32, msg4.len(), 0, 0, 0); }

    // Return 1 for success
    1
}
