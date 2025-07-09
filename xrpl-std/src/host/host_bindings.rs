// This file re-exports the appropriate host bindings implementation based on the target.
// For WASM targets, it uses the WasmHostBindings implementation.
// For non-WASM targets, it uses the TestingHostBindings implementation.

// Import the trait to ensure implementations conform to it
use crate::host::host_bindings_trait::HostBindings;

// For WASM targets, re-export functions from WasmHostBindings
#[cfg(target_arch = "wasm32")]
pub use crate::host::host_bindings_wasm::WasmHostBindings as HostBindingsImpl;

// For non-WASM targets, re-export functions from TestingHostBindings
#[cfg(not(target_arch = "wasm32"))]
pub use crate::host::host_bindings_testing::TestingHostBindings as HostBindingsImpl;

// Re-export all the host functions directly

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_sqn(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
    unsafe { HostBindingsImpl::get_ledger_sqn(out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_parent_ledger_time(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
    unsafe { HostBindingsImpl::get_parent_ledger_time(out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
    unsafe { HostBindingsImpl::get_parent_ledger_hash(out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn cache_ledger_obj(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32 {
    unsafe { HostBindingsImpl::cache_ledger_obj(keylet_ptr, keylet_len, cache_num) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32 {
    unsafe { HostBindingsImpl::get_tx_field(field, out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field2(
    field: i32,
    field2: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe { HostBindingsImpl::get_tx_field2(field, field2, out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field3(
    field: i32,
    field2: i32,
    field3: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe { HostBindingsImpl::get_tx_field3(field, field2, field3, out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field4(
    field: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_tx_field4(field, field2, field3, field4, out_buff_ptr, out_buff_len)
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field5(
    field: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_tx_field5(
            field,
            field2,
            field3,
            field4,
            field5,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::too_many_arguments)]
pub unsafe fn get_tx_field6(
    field: i32,
    field2: i32,
    field3: i32,
    field4: i32,
    field5: i32,
    field6: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_tx_field6(
            field,
            field2,
            field3,
            field4,
            field5,
            field6,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_field(
    field: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe { HostBindingsImpl::get_current_ledger_obj_field(field, out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_field(
    cache_num: i32,
    field: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe { HostBindingsImpl::get_ledger_obj_field(cache_num, field, out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_nested_field(
    locator_ptr: *const u8,
    locator_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_tx_nested_field(locator_ptr, locator_len, out_buff_ptr, out_buff_len)
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_nested_field(
    locator_ptr: *const u8,
    locator_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_current_ledger_obj_nested_field(
            locator_ptr,
            locator_len,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_nested_field(
    cache_num: i32,
    locator_ptr: *const u8,
    locator_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_ledger_obj_nested_field(
            cache_num,
            locator_ptr,
            locator_len,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_array_len(field: i32) -> i32 {
    unsafe { HostBindingsImpl::get_tx_array_len(field) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_array_len(field: i32) -> i32 {
    unsafe { HostBindingsImpl::get_current_ledger_obj_array_len(field) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32 {
    unsafe { HostBindingsImpl::get_ledger_obj_array_len(cache_num, field) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32 {
    unsafe { HostBindingsImpl::get_tx_nested_array_len(locator_ptr, locator_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_nested_array_len(
    locator_ptr: *const u8,
    locator_len: usize,
) -> i32 {
    unsafe { HostBindingsImpl::get_current_ledger_obj_nested_array_len(locator_ptr, locator_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_nested_array_len(
    cache_num: i32,
    locator_ptr: *const u8,
    locator_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_ledger_obj_nested_array_len(cache_num, locator_ptr, locator_len)
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn update_data(data_ptr: *const u8, data_len: usize) -> i32 {
    unsafe { HostBindingsImpl::update_data(data_ptr, data_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn compute_sha512_half(
    data_ptr: *const u8,
    data_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe { HostBindingsImpl::compute_sha512_half(data_ptr, data_len, out_buff_ptr, out_buff_len) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn account_keylet(
    account_ptr: *const u8,
    account_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::account_keylet(account_ptr, account_len, out_buff_ptr, out_buff_len)
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
#[allow(clippy::too_many_arguments)]
pub unsafe fn credential_keylet(
    subject_ptr: *const u8,
    subject_len: usize,
    issuer_ptr: *const u8,
    issuer_len: usize,
    cred_type_ptr: *const u8,
    cred_type_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::credential_keylet(
            subject_ptr,
            subject_len,
            issuer_ptr,
            issuer_len,
            cred_type_ptr,
            cred_type_len,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn escrow_keylet(
    account_ptr: *const u8,
    account_len: usize,
    sequence: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::escrow_keylet(
            account_ptr,
            account_len,
            sequence,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn oracle_keylet(
    account_ptr: *const u8,
    account_len: usize,
    document_id: i32,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::oracle_keylet(
            account_ptr,
            account_len,
            document_id,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft(
    account_ptr: *const u8,
    account_len: usize,
    nft_id_ptr: *const u8,
    nft_id_len: usize,
    out_buff_ptr: *mut u8,
    out_buff_len: usize,
) -> i32 {
    unsafe {
        HostBindingsImpl::get_nft(
            account_ptr,
            account_len,
            nft_id_ptr,
            nft_id_len,
            out_buff_ptr,
            out_buff_len,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace(
    msg_read_ptr: u32,
    msg_read_len: usize,
    data_read_ptr: u32,
    data_read_len: usize,
    as_hex: u32,
) -> i32 {
    unsafe {
        HostBindingsImpl::trace(
            msg_read_ptr,
            msg_read_len,
            data_read_ptr,
            data_read_len,
            as_hex,
        )
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_num(msg_read_ptr: u32, msg_read_len: usize, number: i64) -> i32 {
    unsafe { HostBindingsImpl::trace_num(msg_read_ptr, msg_read_len, number) }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_opaque_float(
    msg_read_ptr: u32,
    msg_read_len: usize,
    opaque_float_ptr: u32,
) -> i32 {
    unsafe { HostBindingsImpl::trace_opaque_float(msg_read_ptr, msg_read_len, opaque_float_ptr) }
}
