use anyhow::Result;
use bs58;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::path::Path;

/// Calculates a fingerprint for a WASM module using the following algorithm:
/// 1. Compute RIPEMD-160 hash of the WASM binary
/// 2. Prepend custom type prefix (0x17)
/// 3. Compute double SHA-256 checksum
/// 4. Append checksum to the prefixed bytes
/// 5. Encode in XRP Ledger Base58 alphabet
///
/// # Arguments
///
/// * `wasm_path` - Path to the WASM file
///
/// # Returns
///
/// * `Result<String>` - The fingerprint string if successful, error otherwise
pub fn calculate_wasm_fingerprint(wasm_path: &Path) -> Result<String> {
    // Read WASM file
    let wasm_bytes = std::fs::read(wasm_path)
        .map_err(|e| anyhow::anyhow!("Failed to read WASM file: {:?}", e))?;

    // Compute RIPEMD-160 hash
    let mut ripemd = Ripemd160::new();
    ripemd.update(&wasm_bytes);
    let ripemd_hash = ripemd.finalize(); // 20 bytes

    // Prepend custom type prefix (0x17) which ensures the result starts with "w" (for 20-byte payload)
    let mut prefixed = vec![0x17];
    prefixed.extend_from_slice(&ripemd_hash);

    // Compute double SHA-256 checksum
    let first_hash = Sha256::digest(&prefixed);
    let second_hash = Sha256::digest(&first_hash);
    let checksum = &second_hash[..4];

    // Append checksum to the prefixed bytes
    let mut with_checksum = prefixed;
    with_checksum.extend_from_slice(checksum);

    // Use XRP Ledger Base58 alphabet
    let xrp_alphabet =
        bs58::Alphabet::new(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz")
            .expect("Provided alphabet is invalid");

    // Encode in Base58
    let fingerprint = bs58::encode(&with_checksum)
        .with_alphabet(&xrp_alphabet)
        .into_string();

    Ok(fingerprint)
}
