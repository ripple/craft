#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host;
use xrpl_std::host::trace::trace_num;

#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    let mul_bytes = [7, 7, 185, 32, 188, 151, 140, 2, 242, 146, 250, 226, 3, 110, 5, 123, 229, 66, 148, 17, 76, 204, 60, 135, 105, 216, 131, 246, 136, 161, 66, 63, 46, 50, 160, 148, 183, 88, 149, 84, 247, 188, 53, 123, 246, 52, 129, 172, 210, 213, 85, 85, 194, 3, 56, 55, 130, 164, 101, 7, 135, 255, 102, 66];
    let scalar_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 238, 117, 236, 77, 51, 91, 71, 50, 139, 18, 62, 39, 137, 67, 136];
    // Mul results: [9, 182, 227, 98, 146, 77, 113, 186, 74, 208, 221, 216, 43, 148, 150, 153, 86, 176, 90, 4, 165, 160, 230, 168, 208, 191, 107, 111, 105, 89, 190, 215, 13, 155, 162, 139, 181, 22, 237, 203, 5, 28, 15, 63, 147, 47, 226, 190, 221, 147, 201, 15, 195, 120, 68, 156, 42, 235, 9, 112, 203, 138, 133, 25]

    let mut mul_buf = [0u8; 64];
    unsafe{
        host::bn254_mul_helper(
            mul_bytes.as_ptr(),
            mul_bytes.len(),
            scalar_bytes.as_ptr(),
            scalar_bytes.len(),
            mul_buf.as_mut_ptr(),
            mul_buf.len(),
        );
    }
    // Print the resulting mul_buf for debugging using the host `trace` function (as hex)
    // let msg = "mul_buf:".as_bytes();
    // unsafe {
    //     // trace(msg_ptr, msg_len, data_ptr, data_len, as_hex)
    //     let _ = host::trace(msg.as_ptr(), msg.len(), mul_buf.as_ptr(), mul_buf.len(), 1);
    // }
    
    // const ORIGIN: [u8; 64] = [
    //     9, 182, 227, 98, 146, 77, 113, 186, 74, 208, 221, 216, 43, 148, 150, 153,
    //     86, 176, 90, 4, 165, 160, 230, 168, 208, 191, 107, 111, 105, 89, 190, 215,
    //     13, 155, 162, 139, 181, 22, 237, 203, 5, 28, 15, 63, 147, 47, 226, 190,
    //     221, 147, 201, 15, 195, 120, 68, 156, 42, 235, 9, 112, 203, 138, 133, 25,
    // ];

    const EXPECTED: [u8; 64] = [
        9, 182, 227, 98, 146, 77, 113, 186, 74, 208, 221, 216, 43, 148, 150, 153,
        86, 176, 90, 4, 165, 160, 230, 168, 208, 191, 107, 111, 105, 89, 190, 215,
        13, 155, 162, 139, 181, 22, 237, 203, 5, 28, 15, 63, 147, 47, 226, 190,
        221, 147, 201, 15, 195, 120, 68, 156, 42, 235, 9, 112, 203, 138, 133, 25,
    ];

    let result = mul_buf.as_slice() == &EXPECTED[..];
    // let result = &ORIGIN[..] == &EXPECTED[..];

    let a = 10;
    let b = 22;
    if result {
        let _ = trace_num("  number:", a as i64);
    } else {
        let _ = trace_num("  number:", b as i64);
    }

    result as i32 // Return 1 if true (successful outcome), 0 if false (failed outcome)
}
