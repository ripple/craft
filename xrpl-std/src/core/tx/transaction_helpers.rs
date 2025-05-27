use crate::core::types::account_id::AccountID;
use crate::core::types::blob::Blob;
use crate::core::types::hash_256::Hash256;
use crate::core::types::public_key::PublicKey;
use crate::host::get_tx_field;
use crate::host::trace::trace;

#[inline(always)]
pub(crate) fn get_u32_field(field_code: i32) -> u32 {
    let mut buffer = [0u8; 4]; // Enough to hold an u32

    unsafe {
        get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len());
    }

    u32::from_le_bytes(buffer)
}

#[inline(always)]
pub(crate) fn get_hash_256_field(field_code: i32) -> Hash256 {
    let mut buffer = [0u8; 32]; // Enough to hold 256 bits (32 bytes)

    unsafe {
        get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len());
    }

    buffer.into() // <-- Move the buffer into an Hash256
}

#[inline(always)]
pub(crate) fn get_public_key_field(field_code: i32) -> PublicKey {
    let mut buffer = [0u8; 33];

    unsafe {
        get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len());
    }

    buffer.into() // <-- Move the buffer into an PublicKey
}

#[inline(always)]
pub(crate) fn get_blob_field(field_code: i32) -> Blob {
    let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

    unsafe {
        let len = get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len());
        Blob {
            data: buffer,
            len: len as usize,
        }
    }
}

#[inline(always)]
pub(crate) fn get_account_id_field(field_code: i32) -> AccountID {
    let mut buffer = [0x00; 20];

    unsafe {
        let result_code = get_tx_field(field_code, buffer.as_mut_ptr(), buffer.len());

        if result_code < 0 {
            let _ = trace("Host function get_current_escrow_finish_field failed!");
            panic!(
                "Failed to get AccountID for field_code={} from host. Error code: {}",
                field_code, result_code
            );
        }

        let bytes_written = result_code as usize;
        assert_eq!(bytes_written, buffer.len());
    }

    buffer.into() // <-- Move the buffer into an AccountID
}
