use std::str;
use xrpl_std::get_current_escrow_destination;
use xrpl_std::host_lib;
use serde_json::Value;
use hex;

// The NFTokenID that the destination must own to finish the escrow
// Example NFTokenID from the xrpl.org documentation
// TODO: Read this from the `Data` field of the EscrowCreate transaction instead.
const REQUIRED_NFT_ID: &str = "000B013A95F14B0044F78A264E41713C64B5F89242540EE208C3098E00000D65";

// Helper function to read data from a pointer and length pair
// This is comparable to the `read_data` function from `xrpl_std` but that function is private.
unsafe fn read_data(ptr: i32) -> Vec<u8> {
    let int_buf = Vec::from_raw_parts(ptr as *mut u8, 8, 8);
    let mut ptr_array: [u8; 4] = [0; 4];
    let mut len_array: [u8; 4] = [0; 4];
    ptr_array.clone_from_slice(&int_buf[0..4]);
    len_array.clone_from_slice(&int_buf[4..8]);
    let ptr = i32::from_le_bytes(ptr_array);
    let len = i32::from_le_bytes(len_array);
    Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize)
}

// Helper function to extract low 96 bits from a hex string
fn low96(nftoken_id: &str) -> &str {
    // The low 96 bits are the last 24 characters of the hex string
    &nftoken_id[nftoken_id.len() - 24..]
}

// Helper function to decrement a hex string by 1
fn decrement_hex(hex: &str) -> String {
    let mut bytes = hex::decode(hex).unwrap_or_default();
    let mut carry = 1;
    
    for byte in bytes.iter_mut().rev() {
        if *byte == 0 && carry == 1 {
            *byte = 0xFF;
        } else {
            *byte = byte.wrapping_sub(carry);
            carry = 0;
        }
    }
    
    hex::encode(bytes)
}

// Helper function to get NFTokenPage entry
unsafe fn get_nftoken_page(key: &[u8]) -> Vec<u8> {
    let key_ptr = key.as_ptr();
    let key_len = key.len();
    let mut fname = String::from("NFTokens");
    let fname_ptr = fname.as_mut_ptr();
    let fname_len = fname.len();
    let r_ptr = host_lib::getLedgerEntryField(0x0050, key_ptr as i32, key_len as i32, fname_ptr as i32, fname_len as i32);
    read_data(r_ptr)
}

// Helper function to check if an NFT is in a page
fn is_nft_in_page(page_entry: &[u8], nftoken_id: &str) -> bool {
    // Parse the page entry as JSON
    let page_str = match str::from_utf8(page_entry) {
        Ok(s) => s,
        Err(_) => return false
    };

    let page_json: Value = match serde_json::from_str(page_str) {
        Ok(v) => v,
        Err(_) => return false
    };

    // Get the NFTokens array from the page
    let nftokens = match page_json.get("NFTokens") {
        Some(Value::Array(arr)) => arr,
        _ => return false
    };

    // Check each NFToken in the page
    for nft in nftokens {
        if let Some(Value::Object(nft_obj)) = nft.get("NFToken") {
            if let Some(Value::String(nft_id)) = nft_obj.get("NFTokenID") {
                if nft_id == nftoken_id {
                    return true;
                }
            }
        }
    }

    false
}

#[no_mangle]
pub fn ready() -> bool {
    unsafe {
        // Get the escrow destination account
        let destination = get_current_escrow_destination();
        let destination_str = match str::from_utf8(&destination) {
            Ok(s) => s,
            Err(_) => return false
        };

        // Get the low 96 bits of the required NFTokenID
        let mut current_low96 = low96(REQUIRED_NFT_ID).to_string();
        
        // Check the 3 pages where the NFT could be located
        for _ in 0..3 {
            // Construct the NFTokenPage ID by concatenating destination account and current low96
            let mut page_id = String::with_capacity(destination_str.len() + current_low96.len());
            page_id.push_str(destination_str);
            page_id.push_str(&current_low96);

            // Get the NFTokenPage entry
            let page_entry = get_nftoken_page(page_id.as_bytes());
            
            // If we found a valid page, check if it contains our NFT
            if !page_entry.is_empty() {
                if is_nft_in_page(&page_entry, REQUIRED_NFT_ID) {
                    return true;
                }
            }

            // Decrement the low96 value to check the previous page
            current_low96 = decrement_hex(&current_low96);
            
            // If we've reached all zeros, we've checked all possible pages
            if current_low96 == "000000000000000000000000" {
                break;
            }
        }

        // If we didn't find the NFT in any of the pages, return false
        false
    }
} 