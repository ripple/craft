use crate::host::Error::InternalError;
use crate::host::{Error, Result, Result::Err, Result::Ok};

pub const INTERNAL_ERROR: i32 = -1;
pub const FIELD_NOT_FOUND: i32 = -2;
pub const BUFFER_TOO_SMALL: i32 = -3;
pub const NO_ARRAY: i32 = -4;
pub const NOT_LEAF_FIELD: i32 = -5;
pub const LOCATOR_MALFORMED: i32 = -6;
pub const SLOT_OUT_RANGE: i32 = -7;
pub const SLOTS_FULL: i32 = -8;
pub const INVALID_SLOT: i32 = -9;
pub const LEDGER_OBJ_NOT_FOUND: i32 = -10;
pub const DECODING_ERROR: i32 = -11;
pub const DATA_FIELD_TOO_LARGE: i32 = -12;
pub const OUT_OF_BOUNDS: i32 = -13;

/// Evaluates a result code and executes a closure on success (result_code > 0).
///
/// # Arguments
///
/// * `result_code` - An integer representing the operation result code
/// * `on_success` - A closure that will be executed if result_code > 0
///
/// # Type Parameters
///
/// * `F` - The type of the closure
/// * `T` - The return type of the closure
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - Contains the value returned by the closure if result_code > 0
/// * `Ok(None)` - If result_code == 0 (no data/empty result)
/// * `Err(Error)` - For negative result codes
///
/// # Note
///
/// This function treats 0 as a valid "no data" state and positive values as success.
#[inline(always)]
pub(crate) fn match_result_code<F, T>(result_code: i32, on_success: F) -> Result<T>
where
    F: FnOnce() -> T,
{
    match result_code {
        code if code >= 0 => Ok(on_success()),
        code => Err(Error::from_code(code)),
    }
}

/// Evaluates a result code against an expected number of bytes and executes a closure on exact match.
///
/// # Arguments
///
/// * `result_code` - An integer representing the operation result code
/// * `expected_num_bytes` - The exact number of bytes expected to have been written
/// * `on_success` - A closure that will be executed if the result code matches expected bytes
///
/// # Type Parameters
///
/// * `F` - The type of the closure
/// * `T` - The return type of the closure
///
/// # Returns
///
/// Returns a `Result<T>` where:
/// * `Ok(T)` - Contains the value returned by the closure if result_code matches expected_num_bytes
/// * `Err(InternalError)` - If result_code is non-negative but doesn't match expected bytes
/// * `Err(Error)` - For negative result codes
///
/// # Note
///
/// This function requires an exact match between the result code and expected byte count,
/// making it suitable for operations where the exact amount of data written is critical.
#[inline(always)]
pub(crate) fn match_result_code_with_expected_bytes<F, T>(
    result_code: i32,
    expected_num_bytes: usize,
    on_success: F,
) -> Result<T>
where
    F: FnOnce() -> T,
{
    match result_code {
        code if code as usize == expected_num_bytes => Ok(on_success()),
        code if code >= 0 => Err(InternalError),
        code => Err(Error::from_code(code)),
    }
}
