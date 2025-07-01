use crate::core::error_codes::match_result_code;

use crate::core::types::amount::amount::Amount;
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
            msg.as_ptr() as u32,
            msg.len(),
            null_ptr as u32,
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
        host::trace(
            msg.as_ptr() as u32,
            msg.len(),
            data_ptr as u32,
            data_len,
            data_repr as _,
        )
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
    let result_code = unsafe { host::trace_num(msg.as_ptr() as u32, msg.len(), number) };
    match_result_code(result_code, || result_code)
}

#[inline(always)]
pub fn trace_amount(msg: &str, amount: &Amount) -> Result<i32> {
    let result_code: i32 = match *amount {
        Amount::XRP { num_drops, .. } => unsafe {
            host::trace_num(msg.as_ptr() as u32, msg.len(), num_drops as i64)
        },
        Amount::IOU {
            exponent,
            mantissa,
            is_positive,
        } => {
            // Create a fixed-size buffer for the formatted message
            let mut data_buffer = [0u8; 256]; // Adjust size as needed
            let mut pos = 0;

            // Copy the original message
            let msg_bytes = msg.as_bytes();
            data_buffer[pos..pos + msg_bytes.len()].copy_from_slice(msg_bytes);
            pos += msg_bytes.len();

            // Add " exponent="
            let exp_prefix = b" exponent=";
            data_buffer[pos..pos + exp_prefix.len()].copy_from_slice(exp_prefix);
            pos += exp_prefix.len();

            // Convert exponent to string and copy
            let exp_bytes: [u8; 1] = exponent.to_le_bytes();
            data_buffer[pos..pos + exp_bytes.len()].copy_from_slice(&exp_bytes);
            pos += exp_bytes.len();

            // Add " mantissa="
            let mantissa_prefix = b" mantissa=";
            data_buffer[pos..pos + mantissa_prefix.len()].copy_from_slice(mantissa_prefix);
            pos += mantissa_prefix.len();

            // Convert mantissa to string and copy
            let mantissa_bytes: [u8; 8] = mantissa.to_le_bytes();
            data_buffer[pos..pos + mantissa_bytes.len()].copy_from_slice(&mantissa_bytes);
            pos += mantissa_bytes.len();

            // Add " is_positive="
            let is_pos_prefix = b" is_positive=";
            data_buffer[pos..pos + is_pos_prefix.len()].copy_from_slice(is_pos_prefix);
            pos += is_pos_prefix.len();

            // Convert is_positive to string and copy
            let is_pos_bytes: [u8; 1] = match is_positive {
                true => [0x01],
                false => [0x00],
            };
            data_buffer[pos..pos + is_pos_bytes.len()].copy_from_slice(&is_pos_bytes);
            // pos += is_pos_bytes.len();
            unsafe {
                let result_code = host::trace(
                    msg.as_ptr() as u32,
                    msg.len(),
                    data_buffer.as_ptr() as u32,
                    data_buffer.len(),
                    DataRepr::AsUTF8 as _,
                );
                result_code
            }
        }
        Amount::MPT { num_units, .. } => unsafe {
            host::trace_num(msg.as_ptr() as u32, msg.len(), num_units as i64)
        },
        Amount::UNKNOWN => {
            panic!("Unknown Amount type")
        }
    };

    match_result_code(result_code, || result_code)
}
