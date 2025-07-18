use crate::core::error_codes::match_result_code;

use crate::host;
use crate::host::Result;
use core::ptr;

/// Data representation
#[derive(Clone, Copy)]
pub enum DataRepr {
    /// As UTF-8
    AsUTF8 = 0,
    /// As hexadecimal
    AsHex = 1,
}

/// Write the contents of a message to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
///
/// # Returns
///
/// Returns an integer representing the result of the operation. A value of `0` or higher signifies
/// the number of message bytes that were written to the trace function. Non-zero values indicate
/// an error (e.g., incorrect buffer sizes).
#[inline(always)] // <-- Inline because this function is very small
pub fn trace(msg: &str) -> Result<i32> {
    let null_ptr: *const u8 = ptr::null::<u8>();

    let result_code = unsafe {
        host::trace(
            msg.as_ptr(),
            msg.len(),
            null_ptr,
            0usize,
            DataRepr::AsUTF8 as _,
        )
    };

    match_result_code(result_code, || result_code)
}

/// Write the contents of a message to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
///
/// # Returns
///
/// Returns an integer representing the result of the operation. A value of `0` or higher signifies
/// the number of message bytes that were written to the trace function. Non-zero values indicate
/// an error (e.g., incorrect buffer sizes).
#[inline(always)] // <-- Inline because this function is very small
pub fn trace_data(msg: &str, data: &[u8], data_repr: DataRepr) -> Result<i32> {
    let result_code = unsafe {
        let data_ptr = data.as_ptr();
        let data_len = data.len();
        host::trace(msg.as_ptr(), msg.len(), data_ptr, data_len, data_repr as _)
    };

    match_result_code(result_code, || result_code)
}

/// Write the contents of a message, and a number, to the xrpld trace log.
///
/// # Parameters
/// * `msg`: A str ref pointing to an array of bytes containing UTF-8 characters.
/// * `number`: A number to emit into the trace logs.
///
/// # Returns
///
/// Returns an integer representing the result of the operation. A value of `0` or higher signifies
/// the number of message bytes that were written to the trace function. Non-zero values indicate
/// an error (e.g., incorrect buffer sizes).
#[inline(always)]
pub fn trace_num(msg: &str, number: i64) -> Result<i32> {
    let result_code = unsafe { host::trace_num(msg.as_ptr(), msg.len(), number) };
    match_result_code(result_code, || result_code)
}

// TODO: Uncomment this line once we have support for floating point numbers (like XFL or similar).
// /// Write a XFL float to the XRPLD trace log
// #[inline(always)]
// pub fn trace_float(msg: &[u8], float: XFL) -> Result<u64> {
//     let res = unsafe { _c::trace_float(msg.as_ptr() as u32, msg.len() as u32, float.0) };
//
//     result_u64(res)
// }
