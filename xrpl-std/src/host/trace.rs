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

/// Write the contents of a buffer to the XRPLD trace log
#[inline(always)] // <-- Inline because this function is very small
pub fn trace_msg(msg: &str) -> Result<i64> {
    let null_ptr: *const u8 = ptr::null::<u8>();

    let res = unsafe {
        host::trace(
            msg.as_ptr() as u32,
            msg.len(),
            null_ptr as u32,
            0usize,
            DataRepr::AsUTF8 as _,
        )
    };

    Result::Ok(res)
}

#[inline(always)] // <-- Inline because this function is very small
pub fn trace_msg_with_data(msg: &str, data: &[u8], data_repr: DataRepr) -> Result<i64> {
    let res = unsafe {
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

    Result::Ok(res)
}

// /// Write an integer to the XRPLD trace log
#[inline(always)]
pub fn trace_num(msg: &str, number: i64) -> Result<i64> {
    let res = unsafe { host::trace_num(msg.as_ptr() as u32, msg.len(), number) };

    Result::Ok(res)
}

// /// Write a XFL float to the XRPLD trace log
// #[inline(always)]
// pub fn trace_float(msg: &[u8], float: XFL) -> Result<u64> {
//     let res = unsafe { _c::trace_float(msg.as_ptr() as u32, msg.len() as u32, float.0) };
//
//     result_u64(res)
// }