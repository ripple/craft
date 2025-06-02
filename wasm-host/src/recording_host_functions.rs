use crate::call_recorder::CallRecorder;
use crate::data_provider::DataProvider;
use crate::host_function_utils::read_utf8_from_wasm;
use log::debug;
use std::cell::RefCell;
use std::rc::Rc;
use wasmedge_sdk::error::{CoreError, CoreExecutionError};
use wasmedge_sdk::{CallingFrame, Instance, WasmValue};

#[allow(dead_code)]
fn get_data(
    in_buf_ptr: i32,
    in_buf_len: i32,
    _caller: &mut CallingFrame,
) -> Result<Vec<u8>, CoreError> {
    let memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("get_data: Error: Failed to get memory instance");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;
    let buffer = memory
        .get_data(in_buf_ptr as u32, in_buf_len as u32)
        .map_err(|e| {
            eprintln!("get_data: Error: Failed to get memory data: {}", e);
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
        })?;
    Ok(buffer)
}

#[allow(dead_code)]
pub fn update_data_with_recording(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
    recorder: Rc<RefCell<CallRecorder>>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;

    // Record the call
    let _return_value = recorder.borrow_mut().record_update_data(data.clone());

    // Original functionality
    _data_provider.set_current_ledger_obj_data(data);

    Ok(vec![])
}

#[allow(dead_code)]
pub fn trace_with_recording(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    inputs: Vec<WasmValue>,
    recorder: Rc<RefCell<CallRecorder>>,
) -> Result<Vec<WasmValue>, CoreError> {
    let msg_read_ptr: u32 = inputs[0].to_i32() as u32;
    let msg_read_len: u32 = inputs[1].to_i32() as u32;
    let data_read_ptr: u32 = inputs[2].to_i32() as u32;
    let data_read_len: u32 = inputs[3].to_i32() as u32;
    let data_as_hex = {
        match inputs[4].to_i32() {
            0 => false,
            1 => true,
            _ => true,
        }
    };

    debug!(
        "trace() params: msg_read_ptr={} msg_read_len={} data_read_ptr={} data_read_len={}",
        msg_read_ptr, msg_read_len, data_read_ptr, data_read_len
    );

    let message = read_utf8_from_wasm(_caller, msg_read_ptr as i32, msg_read_len as i32)?;
    let data = if data_read_len > 0 {
        Some(get_data(
            data_read_ptr as i32,
            data_read_len as i32,
            _caller,
        )?)
    } else {
        None
    };

    // Record the call
    let _return_value = recorder.borrow_mut().record_trace(
        message.clone(),
        data.clone(),
        if data_as_hex { 1 } else { 0 },
    );

    Ok(vec![WasmValue::from_i32(0)])
}
