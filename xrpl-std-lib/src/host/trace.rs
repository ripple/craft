use crate::host;
use crate::host::Result;
use core::ptr;
use log::info;

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
pub fn trace_msg(msg: &str) -> Result<u64> {
    let null_ptr: *const u8 = ptr::null::<u8>();

    let res = unsafe {
        host::trace(
            msg.as_ptr() as u32,
            msg.len() as u32,
            null_ptr as u32,
            0u32,
            DataRepr::AsUTF8 as _,
        )
    };

    res.into()
}

#[inline(always)] // <-- Inline because this function is very small
pub fn trace_msg_with_data(msg: &str, data: &[u8], data_repr: DataRepr) -> Result<u64> {
    let res = unsafe {
        host::trace(
            msg.as_ptr() as u32,
            msg.len() as u32,
            data.as_ptr() as u32,
            data.len() as u32,
            data_repr as _,
        )
    };

    res.into()
}

// /// Write an integer to the XRPLD trace log
// #[inline(always)]
// pub fn trace_num(msg: &[u8], number: i64) -> Result<u64> {
//     let res = unsafe { _c::trace_num(msg.as_ptr() as u32, msg.len() as u32, number) };
//
//     result_u64(res)
// }

// /// Write a XFL float to the XRPLD trace log
// #[inline(always)]
// pub fn trace_float(msg: &[u8], float: XFL) -> Result<u64> {
//     let res = unsafe { _c::trace_float(msg.as_ptr() as u32, msg.len() as u32, float.0) };
//
//     result_u64(res)
// }
