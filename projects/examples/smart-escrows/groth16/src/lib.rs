#![cfg_attr(target_arch = "wasm32", no_std)]


#[cfg(not(target_arch = "wasm32"))]
extern crate std;


use xrpl_wasm_std::host::{bn254_add_helper, bn254_mul_helper, bn254_neg_helper, bn254_pairing_helper};
// use xrpl_std::host::trace::trace_num;
// use xrpl_wasm_std::host::trace;


#[unsafe(no_mangle)]
pub extern "C" fn finish() -> i32 {
    // Example data from Groth16 zk-SNARK
    let vk_bytes: [u8; 832] = [45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251, 177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206, 176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202, 255, 0, 77, 25, 38, 9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128, 166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121, 131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188, 127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63, 140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57, 193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62, 234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200, 25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51, 53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118, 66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92, 217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149, 188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165, 219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211, 123, 76, 230, 204, 1, 102, 250, 125, 170, 3, 176, 60, 213, 239, 250, 149, 172, 155, 238, 148, 241, 245, 239, 144, 113, 87, 189, 164, 129, 44, 207, 11, 76, 145, 244, 43, 182, 41, 248, 58, 28, 26, 160, 133, 255, 40, 23, 154, 18, 217, 34, 219, 160, 84, 112, 87, 204, 170, 233, 75, 157, 105, 207, 170, 78, 96, 64, 31, 234, 127, 62, 3, 51, 17, 12, 16, 19, 79, 32, 11, 25, 246, 73, 8, 70, 213, 24, 201, 174, 168, 104, 54, 110, 251, 114, 40, 202, 92, 145, 210, 148, 13, 3, 7, 98, 30, 96, 243, 31, 203, 247, 87, 232, 55, 232, 103, 23, 131, 24, 131, 45, 11, 45, 116, 213, 158, 47, 234, 28, 113, 66, 223, 24, 125, 63, 198, 211, 18, 172, 154, 37, 220, 213, 225, 168, 50, 169, 6, 26, 8, 44, 21, 221, 29, 97, 170, 156, 77, 85, 53, 5, 115, 157, 15, 93, 101, 220, 59, 228, 2, 90, 167, 68, 88, 30, 190, 122, 217, 23, 49, 145, 28, 137, 133, 105, 16, 111, 245, 162, 211, 15, 62, 238, 43, 35, 198, 14, 233, 128, 172, 212, 7, 7, 185, 32, 188, 151, 140, 2, 242, 146, 250, 226, 3, 110, 5, 123, 229, 66, 148, 17, 76, 204, 60, 135, 105, 216, 131, 246, 136, 161, 66, 63, 46, 50, 160, 148, 183, 88, 149, 84, 247, 188, 53, 123, 246, 52, 129, 172, 210, 213, 85, 85, 194, 3, 56, 55, 130, 164, 101, 7, 135, 255, 102, 66, 11, 202, 54, 226, 203, 230, 57, 75, 62, 36, 151, 81, 133, 63, 150, 21, 17, 1, 28, 113, 72, 227, 54, 244, 253, 151, 70, 68, 133, 15, 195, 71, 46, 222, 124, 154, 207, 72, 207, 58, 55, 41, 250, 61, 104, 113, 78, 42, 132, 53, 212, 250, 109, 184, 247, 244, 9, 193, 83, 177, 252, 223, 155, 139, 27, 138, 249, 153, 219, 251, 179, 146, 124, 9, 28, 194, 170, 242, 1, 228, 136, 203, 172, 195, 226, 198, 182, 251, 90, 37, 249, 17, 46, 4, 242, 167, 43, 145, 162, 106, 169, 46, 27, 111, 87, 34, 148, 159, 25, 42, 129, 200, 80, 213, 134, 216, 26, 96, 21, 127, 62, 156, 240, 79, 103, 156, 204, 214, 43, 95, 73, 78, 214, 116, 35, 91, 138, 193, 117, 11, 223, 213, 167, 97, 95, 0, 45, 74, 29, 206, 254, 221, 208, 110, 218, 90, 7, 108, 205, 13, 47, 229, 32, 173, 32, 32, 170, 185, 203, 186, 129, 127, 203, 185, 168, 99, 184, 167, 111, 248, 143, 20, 249, 18, 197, 231, 22, 101, 178, 173, 94, 130, 15, 28, 60, 13, 93, 157, 160, 250, 3, 102, 104, 67, 205, 228, 232, 46, 134, 155, 165, 37, 47, 206, 60, 37, 213, 148, 3, 32, 177, 196, 212, 147, 33, 75, 252, 255, 116, 244, 37, 246, 254, 140, 13, 7, 179, 7, 72, 45, 139, 200, 187, 47, 54, 8, 246, 130, 135, 170, 1, 189, 11, 105, 232, 9];
    let proof_bytes: [u8; 256] = [13, 122, 222, 219, 24, 176, 11, 131, 44, 145, 82, 41, 157, 173, 57, 229, 174, 222, 191, 248, 145, 2, 90, 142, 0, 212, 117, 73, 96, 215, 203, 122, 15, 214, 129, 98, 16, 97, 102, 154, 119, 152, 166, 150, 227, 119, 168, 177, 32, 53, 183, 154, 70, 232, 220, 253, 39, 36, 213, 95, 96, 147, 223, 24, 23, 29, 232, 105, 122, 30, 153, 220, 48, 61, 246, 128, 76, 62, 109, 215, 250, 215, 73, 200, 184, 138, 98, 199, 167, 185, 189, 59, 137, 87, 139, 207, 10, 218, 198, 69, 237, 32, 140, 139, 63, 233, 218, 125, 120, 58, 3, 203, 79, 133, 232, 242, 167, 245, 194, 153, 61, 51, 226, 165, 150, 241, 27, 36, 21, 1, 200, 78, 224, 39, 133, 162, 52, 77, 122, 50, 33, 30, 15, 41, 161, 242, 189, 28, 245, 150, 109, 7, 29, 105, 57, 3, 204, 125, 45, 82, 10, 57, 254, 140, 190, 141, 123, 49, 133, 60, 7, 139, 233, 158, 14, 4, 121, 135, 27, 217, 14, 89, 9, 136, 132, 142, 38, 215, 126, 30, 48, 240, 44, 218, 50, 20, 200, 45, 102, 161, 73, 124, 162, 59, 105, 135, 23, 248, 68, 112, 213, 189, 35, 19, 89, 20, 136, 146, 19, 20, 5, 21, 87, 156, 45, 13, 132, 182, 164, 114, 16, 10, 61, 129, 232, 45, 224, 255, 231, 137, 205, 19, 60, 62, 75, 170, 93, 214, 245, 71, 242, 136, 163, 115, 25, 193];
    let public_bytes: [u8; 160] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 238, 117, 236, 77, 51, 91, 71, 50, 139, 18, 62, 39, 137, 67, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 238, 70, 199, 68, 126, 254, 33, 32, 51, 28, 212, 51, 203, 119, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 232, 249, 100, 175, 51, 167, 132, 174, 55, 7, 17, 163, 216, 231, 236, 118, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 102, 188, 152, 112, 181, 201, 185, 33, 46, 6, 252, 158, 164, 216, 161, 4, 68, 110, 102, 211, 0, 235, 127, 180, 92, 151, 38, 187, 83, 199, 147, 221, 164, 7, 166, 46, 150, 1, 97, 139, 180, 60, 92, 20, 101, 122, 192];
        
    const SCALAR_SIZE_UNCOMPRESSED: usize = 32;
    const G1_SIZE_UNCOMPRESSED: usize = 64;
    const G2_SIZE_UNCOMPRESSED: usize = 128;
    const PAIR_BYTES_LEN: usize = 4 * (G1_SIZE_UNCOMPRESSED + G2_SIZE_UNCOMPRESSED);  // 768

    // let mut par_vec_ic: [u8; 128] = [0u8; 128];
    let mut par_vec_ic: [u8; 384] = [0u8; 384];
    par_vec_ic.copy_from_slice(&vk_bytes[448..]);       // Each ic[i] has a compressed length 32
    let ic_zero = &par_vec_ic[0..G1_SIZE_UNCOMPRESSED];   // Get ic[0]
    let mut mul_buf = [0u8; G1_SIZE_UNCOMPRESSED];
    let mut add_buf = [0u8; G1_SIZE_UNCOMPRESSED];
    add_buf.copy_from_slice(&ic_zero);
    
    // Compute linear combination of ic and public input
    for (i, input_bytes) in public_bytes.chunks(SCALAR_SIZE_UNCOMPRESSED).enumerate() {
        let start = G1_SIZE_UNCOMPRESSED * (i + 1);
        let end = start + G1_SIZE_UNCOMPRESSED;
        let ic_bytes = &par_vec_ic[start..end];         // Iterate the ic[i]

        let mut ic_bytes_le = [0u8; G1_SIZE_UNCOMPRESSED];
        ic_bytes_le.copy_from_slice(ic_bytes);

        // Convert scalar to little-endian
        let mut scalar_le = [0u8; SCALAR_SIZE_UNCOMPRESSED];
        scalar_le.copy_from_slice(input_bytes);
        unsafe{
            bn254_mul_helper(
                ic_bytes_le.as_ptr(),
                ic_bytes_le.len(),
                scalar_le.as_ptr(),
                scalar_le.len(),
                mul_buf.as_mut_ptr(),
                mul_buf.len(),
            );
        }
        unsafe{
            bn254_add_helper(
                add_buf.as_ptr(),
                add_buf.len(),
                mul_buf.as_ptr(),
                mul_buf.len(),
                add_buf.as_mut_ptr(),
                add_buf.len(),
            );
        }
    }

    let vk_x_bytes = add_buf.clone();

    // Pairing check
    let proof_a_bytes = &proof_bytes[0..G1_SIZE_UNCOMPRESSED];
    let proof_b_bytes = &proof_bytes[G1_SIZE_UNCOMPRESSED..G1_SIZE_UNCOMPRESSED+G2_SIZE_UNCOMPRESSED];
    let proof_c_bytes = &proof_bytes[G1_SIZE_UNCOMPRESSED+G2_SIZE_UNCOMPRESSED..];

    // Negation solution 1: use host function ec_negation()
    let mut proof_a_buf = [0u8; G1_SIZE_UNCOMPRESSED];
    proof_a_buf.copy_from_slice(proof_a_bytes);

    // let hex_str: String = proof_a_buf.iter().map(|b| format!("{:02X}", b)).collect();
    // println!("+++++++++test_vk value: {:?}", hex_str);
    let mut neg_proof_a_bytes = [0u8; G1_SIZE_UNCOMPRESSED];
    unsafe{
        let _ = bn254_neg_helper(                    // negate a
            proof_a_buf.as_ptr(),
            proof_a_buf.len(),
            neg_proof_a_bytes.as_mut_ptr(),
            neg_proof_a_bytes.len(),
        );
    }
    // Negation solution 2: flip the MSB of the last byte to negate the value.
    // CAVEAT: need further investigation of serialize_compressed method in arkworks to ensure
    // that the sign bit is the MSB of the last byte
    // let mut neg_proof_a_bytes = [0u8; G1_SIZE_COMPRESSED];
    // neg_proof_a_bytes.copy_from_slice(proof_a_bytes);
    // neg_proof_a_bytes[G1_SIZE_COMPRESSED-1] ^= 0b1000_0000;
    // println!("negated proof_a_bytes is: {:?}", &neg_proof_a_bytes);

    let alpha_bytes = &vk_bytes[0..G1_SIZE_UNCOMPRESSED];
    let beta_bytes  = &vk_bytes[G1_SIZE_UNCOMPRESSED..G1_SIZE_UNCOMPRESSED + G2_SIZE_UNCOMPRESSED];
    let gamma_bytes = &vk_bytes[G1_SIZE_UNCOMPRESSED + G2_SIZE_UNCOMPRESSED..G1_SIZE_UNCOMPRESSED + 2*G2_SIZE_UNCOMPRESSED];
    let delta_bytes = &vk_bytes[G1_SIZE_UNCOMPRESSED + 2*G2_SIZE_UNCOMPRESSED..G1_SIZE_UNCOMPRESSED + 3*G2_SIZE_UNCOMPRESSED];

    let mut input_bytes = [0u8; PAIR_BYTES_LEN];
    let mut offset = 0;
    // 1. (neg a, b)
    input_bytes[offset..offset + G1_SIZE_UNCOMPRESSED].copy_from_slice(&neg_proof_a_bytes);
    offset += G1_SIZE_UNCOMPRESSED;
    input_bytes[offset..offset + G2_SIZE_UNCOMPRESSED].copy_from_slice(&proof_b_bytes);
    offset += G2_SIZE_UNCOMPRESSED;

    // 2. (alpha, beta)
    input_bytes[offset..offset + G1_SIZE_UNCOMPRESSED].copy_from_slice(&alpha_bytes);
    offset += G1_SIZE_UNCOMPRESSED;
    input_bytes[offset..offset + G2_SIZE_UNCOMPRESSED].copy_from_slice(&beta_bytes);
    offset += G2_SIZE_UNCOMPRESSED;

    // 3. (vk_x, gamma)
    input_bytes[offset..offset + G1_SIZE_UNCOMPRESSED].copy_from_slice(&vk_x_bytes);
    offset += G1_SIZE_UNCOMPRESSED;
    input_bytes[offset..offset + G2_SIZE_UNCOMPRESSED].copy_from_slice(&gamma_bytes);
    offset += G2_SIZE_UNCOMPRESSED;

    // 4. (c, delta)
    input_bytes[offset..offset + G1_SIZE_UNCOMPRESSED].copy_from_slice(&proof_c_bytes);
    offset += G1_SIZE_UNCOMPRESSED;
    input_bytes[offset..offset + G2_SIZE_UNCOMPRESSED].copy_from_slice(&delta_bytes);

   // trace_data("  Data: ", &input_bytes, DataRepr::AsHex);

    let result = unsafe{
        bn254_pairing_helper(input_bytes.as_ptr(), PAIR_BYTES_LEN)
    };

    result as i32 
}
