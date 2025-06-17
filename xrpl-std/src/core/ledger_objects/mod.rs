pub mod account;
pub mod current_escrow;
pub mod escrow;
pub mod nft;
pub mod traits;

pub mod current_ledger_object {
    use crate::core::error_codes::{
        match_result_code, match_result_code_with_expected_bytes,
        match_result_code_with_expected_bytes_optional,
    };
    use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::core::types::blob::Blob;
    use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
    use crate::host::{Result, get_current_ledger_obj_field, to_non_optional};

    /// Retrieves an AccountID field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline(always)]
    pub fn get_account_id_field(field_code: i32) -> Result<AccountID> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code_with_expected_bytes(result_code, buffer.len(), || buffer.into())
    }

    //     pub fn get_amount_field(register: i32, field_code: i32) -> Result<u64> {
    //         let mut buffer = [0u8; 8]; // Enough to hold a u64
    //
    //         let result_code = unsafe {
    //             get_current_ledger_obj_field(register, field_code, buffer.as_mut_ptr(), buffer.len())
    //         };
    //
    //         match_result_code_with_expected_bytes(result_code, 8, || {
    // }

    #[inline]
    pub fn get_u32_field(field_code: i32) -> Result<u32> {
        to_non_optional(get_u32_field_optional(field_code))
    }

    #[inline]
    pub fn get_u32_field_optional(field_code: i32) -> Result<Option<u32>> {
        let mut buffer = [0u8; 4]; // Enough to hold an u32

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code_with_expected_bytes_optional(result_code, 4, || {
            Some(u32::from_le_bytes(buffer)) // <-- Move the buffer into an AccountID
        })
    }

    #[inline]
    pub fn get_hash_256_field(field_code: i32) -> Result<Hash256> {
        to_non_optional(get_hash_256_field_optional(field_code))
    }

    #[inline]
    pub fn get_hash_256_field_optional(field_code: i32) -> Result<Option<Hash256>> {
        let mut buffer = [0u8; HASH256_SIZE]; // Enough to hold 256 bits (32 bytes)

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer)) // <-- Move the buffer into an Hash256
        })
    }

    #[inline]
    pub fn get_blob_field(field_code: i32) -> Result<Blob> {
        to_non_optional(get_blob_field_optional(field_code))
    }

    #[inline]
    pub fn get_blob_field_optional(field_code: i32) -> Result<Option<Blob>> {
        let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

        let result_code =
            unsafe { get_current_ledger_obj_field(field_code, buffer.as_mut_ptr(), buffer.len()) };

        match_result_code(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
    }
}

pub mod ledger_object {
    use crate::core::error_codes::{
        match_result_code, match_result_code_with_expected_bytes,
        match_result_code_with_expected_bytes_optional,
    };
    use crate::core::types::account_id::{ACCOUNT_ID_SIZE, AccountID};
    use crate::core::types::blob::Blob;
    use crate::core::types::hash_256::{HASH256_SIZE, Hash256};
    use crate::host;
    use crate::host::{Result, get_ledger_obj_field, to_non_optional};

    /// Retrieves an AccountID field from the current ledger object.
    ///
    /// # Arguments
    ///
    /// * `field_code` - The field code identifying which AccountID field to retrieve
    ///
    /// # Returns
    ///
    /// Returns a `Result<AccountID>` where:
    /// * `Ok(AccountID)` - The account identifier for the specified field
    /// * `Err(Error)` - If the field cannot be retrieved or has unexpected size
    #[inline(always)]
    pub fn get_account_id_field(register: i32, field_code: i32) -> Result<AccountID> {
        let mut buffer = [0x00; ACCOUNT_ID_SIZE];

        let result_code = unsafe {
            host::get_ledger_obj_field(register, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes(result_code, buffer.len(), || buffer.into())
    }

    //     pub fn get_amount_field(register: i32, field_code: i32) -> Result<u64> {
    //         let mut buffer = [0u8; 8]; // Enough to hold a u64
    //
    //         let result_code = unsafe {
    //             host::get_ledger_obj_field(register, field_code, buffer.as_mut_ptr(), buffer.len())
    //         };
    //
    //         match_result_code_with_expected_bytes(result_code, 8, || {
    // }

    #[inline]
    pub fn get_u32_field(register_num: i32, field_code: i32) -> Result<u32> {
        to_non_optional(get_u32_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_u32_field_optional(register_num: i32, field_code: i32) -> Result<Option<u32>> {
        let mut buffer = [0u8; 4]; // Enough to hold an u32

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes_optional(result_code, 4, || {
            Some(u32::from_le_bytes(buffer)) // <-- Move the buffer into an AccountID
        })
    }

    #[inline]
    pub fn get_hash_256_field(register_num: i32, field_code: i32) -> Result<Hash256> {
        to_non_optional(get_hash_256_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_hash_256_field_optional(
        register_num: i32,
        field_code: i32,
    ) -> Result<Option<Hash256>> {
        let mut buffer = [0u8; HASH256_SIZE]; // Enough to hold 256 bits (32 bytes)

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code_with_expected_bytes(result_code, HASH256_SIZE, || {
            Some(Hash256(buffer)) // <-- Move the buffer into an Hash256
        })
    }

    #[inline]
    pub fn get_blob_field(register_num: i32, field_code: i32) -> Result<Blob> {
        to_non_optional(get_blob_field_optional(register_num, field_code))
    }

    #[inline]
    pub fn get_blob_field_optional(register_num: i32, field_code: i32) -> Result<Option<Blob>> {
        let mut buffer = [0u8; 1024]; // Enough to hold the largest field, which is a memo.

        let result_code = unsafe {
            get_ledger_obj_field(register_num, field_code, buffer.as_mut_ptr(), buffer.len())
        };

        match_result_code(result_code, || {
            Some(Blob {
                data: buffer,
                len: result_code as usize,
            })
        })
    }
}
