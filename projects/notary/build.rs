use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Require NOTARY_ACCOUNT_R to be set at build time for local/dev builds.
    // In CI (e.g., GitHub Actions) fall back to the master account so example build passes.
    // Example: NOTARY_ACCOUNT_R=rPPL... craft build notary
    let raddr = match env::var("NOTARY_ACCOUNT_R") {
        Ok(v) => v,
        Err(_) => {
            if env::var("GITHUB_ACTIONS").is_ok() || env::var("CI").is_ok() {
                "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh".to_string()
            } else {
                panic!("Environment variable NOTARY_ACCOUNT_R must be set to a classic r-address")
            }
        }
    };

    let notary_bytes = decode_classic_address_to_20bytes(&raddr)
        .expect("Invalid NOTARY_ACCOUNT_R: must be a valid classic r-address");

    assert_eq!(notary_bytes.len(), 20, "NOTARY_ACCOUNT must be 20 bytes");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let dest_path = out_dir.join("notary_generated.rs");
    let mut content = String::new();

    content.push_str("// Auto-generated. Do not edit by hand.\n");
    content.push_str("#[allow(dead_code)]\n");
    content.push_str("pub const NOTARY_ACCOUNT: [u8; 20] = [");
    for (i, b) in notary_bytes.iter().enumerate() {
        if i > 0 {
            content.push_str(", ");
        }
        content.push_str(&format!("0x{:02x}", b));
    }
    content.push_str("];\n");

    fs::write(dest_path, content).expect("Failed to write notary_generated.rs");
}

// Decode a classic address (r-address) to a 20-byte AccountID
fn decode_classic_address_to_20bytes(addr: &str) -> Option<Vec<u8>> {
    if !addr.starts_with('r') {
        return None;
    }
    let alphabet =
        bs58::Alphabet::new(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz").ok()?;
    let full = bs58::decode(addr)
        .with_alphabet(&alphabet)
        .into_vec()
        .ok()?;
    if full.len() < 1 + 20 + 4 {
        return None;
    }
    // Version byte should be 0x00 for classic AccountID
    if full[0] != 0x00 {
        return None;
    }
    // Split payload and checksum
    let (payload, checksum) = full.split_at(full.len() - 4);
    // Verify checksum: double SHA-256 of payload, take first 4 bytes
    use sha2::{Digest, Sha256};
    let first = Sha256::digest(payload);
    let second = Sha256::digest(&first);
    if &second[0..4] != checksum {
        return None;
    }
    // Payload is version (1) + 20 bytes account id
    if payload.len() != 1 + 20 {
        return None;
    }
    Some(payload[1..].to_vec())
}

// Build-deps in build script context
extern crate bs58;
extern crate hex;
extern crate sha2;
