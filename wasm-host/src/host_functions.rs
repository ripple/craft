use wasmedge_sdk::{CallingFrame, Instance, WasmValue};
use wasmedge_sdk::error::{CoreError, CoreExecutionError};
use crate::mock_data::{Hash256, MockData, Keylet, DataSource};

const LOCATOR_BUFFER_SIZE: usize = 64;
const NUM_SLOTS: usize = 256;

type AccountId = Vec<u8>; //TODO size

pub enum HostError {
    InternalError = -1,
    FieldNotFound = -2,
    BufferTooSmall = -3,
    NotArray = -4,
    NotLeafField = -5,
    LocatorMalformed = -6,
    SlotOutRange = -7,
    SlotsFull = -8,
    InvalidSlot = -9,
    LedgerObjNotFound = -10,
}

pub struct LocatorUnpacker {
    buffer: Vec<u8>,
    cur_buffer_index: usize,
    packed_bytes: usize,
}

impl LocatorUnpacker {
    pub fn from_bytes(buffer: Vec<u8>, packed_bytes: usize) -> Option<LocatorUnpacker> {
        if packed_bytes > LOCATOR_BUFFER_SIZE || packed_bytes == 0 || packed_bytes % 4 != 0 {
            None
        } else {
            Some(LocatorUnpacker {
                buffer,
                cur_buffer_index: 0,
                packed_bytes,
            })
        }
    }

    pub fn unpack(&mut self) -> Option<i32> {
        if self.cur_buffer_index + 4 > self.packed_bytes {
            return None;
        }
        let mut bytes: [u8; 4] = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4]);
        self.cur_buffer_index += 4;
        Some(i32::from_le_bytes(bytes))
    }
}

pub fn unpack_locator(buffer: Vec<u8>, packed_bytes: usize) -> Result<Vec<i32>, HostError> {
    let mut unpacker = LocatorUnpacker::from_bytes(buffer, packed_bytes)
        .ok_or(HostError::LocatorMalformed)?;

    let mut result = vec![];
    while let Some(slot) = unpacker.unpack() {
        result.push(slot);
    }
    Ok(result)
}

pub struct DataProvider {
    data_source: MockData,
    next_slot: usize,
    slots: [Keylet; NUM_SLOTS],
}

impl DataProvider {
    pub fn new(data_source: MockData) -> Self {
        let slots: [Hash256; 256] = core::array::from_fn(|_| Hash256::default());
        Self {
            data_source,
            next_slot: 1,
            slots,
        }
    }

    pub fn slot_set(&mut self, keylet: Keylet, mut slot: usize) -> i32 {
        if slot == 0 {
            if self.next_slot >= NUM_SLOTS {
                return HostError::SlotsFull as i32;
            }
            slot = self.next_slot;
            self.next_slot += 1;
        } else if slot >= NUM_SLOTS {
            return HostError::InvalidSlot as i32;
        }

        if self.data_source.obj_exist(&keylet) {
            self.slots[slot] = keylet;
            slot as i32
        } else {
            HostError::LedgerObjNotFound as i32
        }
    }

    pub fn slot_get(&self, slot: usize) -> Option<&Keylet> {
        if slot == 0 || slot >= self.next_slot {
            None
        } else {
            Some(&self.slots[slot])
        }
    }

    pub fn get_field_value(&self, source: DataSource, idx_fields: Vec<i32>, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_field_value(source, idx_fields);
        Self::fill_buf(field_result, buf_cap)
    }

    pub fn get_ledger_sqn(&self, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_ledger_sqn();
        Self::fill_buf(field_result, buf_cap)
    }

    pub fn get_parent_ledger_time(&self, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_parent_ledger_time();
        Self::fill_buf(field_result, buf_cap)
    }

    pub fn get_parent_ledger_hash(&self, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_parent_ledger_hash();
        Self::fill_buf(field_result, buf_cap)
    }

    pub fn fill_buf(field_result: Option<&serde_json::Value>, buf_cap: usize) -> (i32, Vec<u8>) {
        let mut buf = Vec::with_capacity(buf_cap);
        match field_result {
            Some(value) => {
                match value {
                    serde_json::Value::Number(n) => {
                        if n.is_i64() {
                            let num = n.as_i64().unwrap();
                            if buf_cap == 4 {
                                // Safe cast to i32
                                if num > i32::MAX as i64 || num < i32::MIN as i64 {
                                    return (HostError::BufferTooSmall as i32, buf);
                                }
                                let bytes = (num as i32).to_le_bytes();
                                buf[..4].copy_from_slice(&bytes);
                                (4, buf)
                            } else {
                                let bytes = num.to_le_bytes();
                                if bytes.len() > buf_cap {
                                    return (HostError::BufferTooSmall as i32, buf);
                                }
                                buf[..bytes.len()].copy_from_slice(&bytes);
                                (bytes.len() as i32, buf)
                            }
                        } else if n.is_u64() {
                            let num = n.as_u64().unwrap();
                            if buf_cap == 4 {
                                // Safe cast to u32
                                if num > u32::MAX as u64 {
                                    return (HostError::BufferTooSmall as i32, buf);
                                }
                                let bytes = (num as u32).to_le_bytes();
                                buf[..4].copy_from_slice(&bytes);
                                (4, buf)
                            } else {
                                let bytes = num.to_le_bytes();
                                if bytes.len() > buf_cap {
                                    return (HostError::BufferTooSmall as i32, buf);
                                }
                                buf[..bytes.len()].copy_from_slice(&bytes);
                                (bytes.len() as i32, buf)
                            }
                        } else {
                            return (HostError::InternalError as i32, buf);
                        }
                    }
                    serde_json::Value::String(s) => {
                        let bytes = s.as_bytes();
                        if bytes.len() > buf_cap {
                            return (HostError::BufferTooSmall as i32, buf);
                        }
                        buf[..bytes.len()].copy_from_slice(bytes);
                        (bytes.len() as i32, buf)
                    }
                    _ => (HostError::InternalError as i32, buf)
                }
            }
            None => (HostError::FieldNotFound as i32, buf)
        }
    }
}

fn get_data(
    in_buf_ptr: i32,
    in_buf_len: i32,
    _caller: &mut CallingFrame,
) -> Result<Vec<u8>, CoreError> {
    let mut memory = _caller.memory_mut(0).ok_or_else(|| {
        eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;
    let buffer = memory.get_data(in_buf_ptr as u32, in_buf_len as u32).map_err(|e| {
        eprintln!("get_tx_hash_helper: Error: Failed to get memory data: {}", e);
        CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
    })?;
    Ok(buffer)
}

fn get_keylet(
    in_buf_ptr: i32,
    in_buf_len: i32,
    _caller: &mut CallingFrame,
) -> Result<Keylet, CoreError> {
    get_data(in_buf_ptr, in_buf_len, _caller)
}

fn get_account_id(
    in_buf_ptr: i32,
    in_buf_len: i32,
    _caller: &mut CallingFrame,
) -> Result<AccountId, CoreError> {
    get_data(in_buf_ptr, in_buf_len, _caller)
}

fn get_locator_data(in_buf_ptr: i32,
                    in_buf_len: i32,
                    _caller: &mut CallingFrame,
) -> Result<Vec<u8>, CoreError> {
    get_data(in_buf_ptr, in_buf_len, _caller)
}

fn set_data(
    dp_res: i32,
    out_buf_ptr: i32,
    data_to_write: Vec<u8>,
    _caller: &mut CallingFrame,
) -> Result<(), CoreError> {
    if dp_res > 0 {
        let mut memory = _caller.memory_mut(0).ok_or_else(|| {
            eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
        })?;
        memory.set_data(&data_to_write, out_buf_ptr as u32).map_err(|e| {
            eprintln!("get_tx_hash_helper: Error: Failed to set memory data: {}", e);
            CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
        })?;
    }
    Ok(())
}

pub fn get_ledger_sqn(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let out_buf_ptr: i32 = _inputs[0].to_i32();
    let out_buf_cap: i32 = _inputs[1].to_i32();
    let dp_res = _data_provider.get_ledger_sqn(out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_parent_ledger_time(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let out_buf_ptr: i32 = _inputs[0].to_i32();
    let out_buf_cap: i32 = _inputs[1].to_i32();
    let dp_res = _data_provider.get_parent_ledger_time(out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_parent_ledger_hash(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let out_buf_ptr: i32 = _inputs[0].to_i32();
    let out_buf_cap: i32 = _inputs[1].to_i32();
    let dp_res = _data_provider.get_parent_ledger_hash(out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn ledger_slot_set(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_cap: i32 = _inputs[1].to_i32();
    let slot_num: i32 = _inputs[2].to_i32();
    let keylet = get_keylet(in_buf_ptr, in_buf_cap, _caller)?;
    let dp_res = _data_provider.slot_set(keylet, slot_num as usize);
    Ok(vec![WasmValue::from_i32(dp_res)])
}

pub fn get_tx_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let out_buf_ptr: i32 = _inputs[1].to_i32();
    let out_buf_cap: i32 = _inputs[2].to_i32();
    let dp_res = _data_provider.get_field_value(DataSource::Tx, vec![field], out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_current_ledger_obj_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    let out_buf_ptr: i32 = _inputs[1].to_i32();
    let out_buf_cap: i32 = _inputs[2].to_i32();
    let dp_res = _data_provider.get_field_value(DataSource::CurrentLedgerObj, vec![field], out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_ledger_obj_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let field: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();
    let keylet = match _data_provider.slot_get(slot as usize){
        None => {return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)])}
        Some(key) => {key.clone()}
    };
    let dp_res = _data_provider.get_field_value(DataSource::KeyletLedgerObj(keylet), vec![field], out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}



// pub fn get_tx_nested_field(locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
// pub fn get_current_ledger_obj_nested_field(locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
// pub fn get_ledger_obj_nested_field(slot: i32, locator_ptr: *const u8, locator_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
//
// pub fn get_tx_array_len(field: i32) -> i32;
// pub fn get_current_ledger_obj_array_len(field: i32) -> i32;
// pub fn get_ledger_obj_array_len(slot: i32, field: i32) -> i32;
//
// pub fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;
// pub fn get_current_ledger_obj_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;
// pub fn get_ledger_obj_nested_array_len(slot: i32, locator_ptr: *const u8, locator_len: usize) -> i32;
//
// pub fn updateData(data_ptr: *const u8, data_len: usize);
//
// pub fn computeSha512HalfHash(data_ptr: *const u8, data_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
// pub fn accountKeylet(account_ptr: *const u8, account_len: usize, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;
//


// pub fn get_current_escrow_finish_field(
//     _: &mut (),
//     _inst: &mut Instance,
//     _caller: &mut CallingFrame,
//     inputs: Vec<WasmValue>,
// ) -> Result<Vec<WasmValue>, CoreError> {
//     // This block simulates a Transaction
//     let apply_ctx = ApplyContext {
//         tx: get_default_escrow_finish(),
//     };
//
//     // check the number of inputs
//     if inputs.len() != 3 {
//         return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
//     }
//
//     // This a pointer to the memory allocated by WASM (i.e., guest memory)
//     let guest_write_ptr = if inputs[0].ty() == ValType::I32 {
//         inputs[0].to_i32()
//     } else {
//         return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
//     };
//
//     let guest_write_ptr_u32: u32 = guest_write_ptr as u32;
//     // println!("guest_write_ptr_u32: {}", guest_write_ptr_u32);
//
//     let guest_write_len = if inputs[1].ty() == ValType::I32 {
//         inputs[1].to_i32() as usize
//     } else {
//         return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
//     };
//     // println!("guest_write_len: {}", guest_write_len);
//
//     // parse the third input of WebAssembly value type into Rust built-in value type
//     let field_code = if inputs[2].ty() == ValType::I32 {
//         inputs[2].to_i32()
//     } else {
//         return Err(CoreError::Execution(CoreExecutionError::FuncSigMismatch));
//     };
//     // println!("field_code: {} (0x{:0x})", field_code, field_code);
//
//     // 2. Context Retrieval (Replaces reinterpret_cast<hook::HookContext*>(data_ptr) and HOOK_SETUP)
//     // Get memory. Assumes memory index 0.
//     let mut memory = _caller.memory_mut(0).ok_or_else(|| {
//         eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
//         CoreError::Execution(CoreExecutionError::MemoryOutOfBounds)
//     })?;
//
//     // 4. Bounds Check (Matches NOT_IN_BOUNDS)
//     let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)
//
//     // Check if write_ptr + tx_id_size overflows or goes out of bounds
//     // Using checked_add to prevent overflow issues during check.
//     let end_ptr = match guest_write_ptr_u32.checked_add(guest_write_len as u32) {
//         Some(end) => end,
//         None => {
//             println!("get_tx_hash_helper: Out of bounds (pointer + size overflow)");
//             return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
//         }
//     };
//
//     if end_ptr > memory_size {
//         println!(
//             "get_tx_hash_helper: Out of bounds (ptr {} + size {} > memory {})",
//             guest_write_ptr_u32, guest_write_len, memory_size
//         );
//         // Return OUT_OF_BOUNDS as i64
//         return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
//     }
//
//     // Write into WASM Memory
//     let data_to_write: Vec<u8> = get_field_bytes(apply_ctx.tx, field_code)?;
//     // This is unsafe if an emulated VM supports 128-bit addressing
//     let data_to_write_len = data_to_write.as_slice().len(); //as u64 as i64;
//
//     // println!("Data: {}", hex::encode(&data_to_write));
//     // println!("Guest write_ptr: {}", guest_write_ptr_u32);
//     // println!("Guest write_len: {}", guest_write_len); // Add logging
//     // println!("Actual data len: {}", data_to_write_len); // Add logging
//
//     // This check ensures the _actual_ data len doesn't exceed what the guest is indicating.
//     if data_to_write_len > guest_write_len as usize {
//         eprintln!(
//             "Error: Data size ({}) exceeds guest buffer size ({}).",
//             data_to_write.len(),
//             guest_write_len
//         );
//         // Return an appropriate error code to WASM.
//         // Using a custom error code might be better than reusing OUT_OF_BOUNDS.
//         // For example, define a BUFFER_TOO_SMALL = -7 or similar.
//         // Let's use OUT_OF_BOUNDS for now as an example if you haven't defined others.
//         // Or a new BUFFER_TOO_SMALL code
//         return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
//     }
//
//     debug!(
//         "WRITING (ptr={} len={}) DATA: {:?}",
//         guest_write_ptr_u32,
//         &data_to_write.len(),
//         data_to_write
//     );
//
//     let result = memory.set_data(&data_to_write, guest_write_ptr_u32);
//
//     match result {
//         Ok(()) => {
//             debug!(
//                 "Success Wasm memory wrote (ptr={} len={}) DATA: {:?}",
//                 guest_write_ptr_u32, guest_write_len, data_to_write
//             );
//         }
//         Err(error) => {
//             eprintln!("Error: Wasm memory write failed: {}", error);
//             return Err(CoreError::Execution(CoreExecutionError::MemoryOutOfBounds));
//         }
//     }
//
//     Ok(vec![WasmValue::from_i64(data_to_write_len as i64)])
// }

// pub fn get_tx_hash(
//     _: &mut (),
//     _inst: &mut Instance,
//     _caller: &mut CallingFrame,
//     inputs: Vec<WasmValue>,
// ) -> Result<Vec<WasmValue>, CoreError> {
//     // This block simulates a Transaction
//     let apply_ctx = ApplyContext {
//         tx: get_default_escrow_finish(),
//     };
//
//     // check the number of inputs
//     if inputs.len() != 1 {
//         return Err(CoreError::Execution(
//             wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
//         ));
//     }
//
//     // parse the first input of WebAssembly value type into Rust built-in value type
//     let guest_write_ptr = if inputs[0].ty() == ValType::I32 {
//         inputs[0].to_i32()
//     } else {
//         return Err(CoreError::Execution(
//             wasmedge_sdk::error::CoreExecutionError::FuncSigMismatch,
//         ));
//     };
//
//     let guest_write_ptr_u32: u32 = guest_write_ptr as u32;
//
//     // 2. Context Retrieval (Replaces reinterpret_cast<hook::HookContext*>(data_ptr) and HOOK_SETUP)
//     // Get memory. Assumes memory index 0.
//     let mut memory = _caller.memory_mut(0).ok_or_else(|| {
//         eprintln!("get_tx_hash_helper: Error: Failed to get memory instance");
//         CoreError::Execution(wasmedge_sdk::error::CoreExecutionError::MemoryOutOfBounds)
//     })?;
//     debug!("memory.size (pages): {}", memory.size());
//
//     // 3. Get Transaction ID (Matches C++ logic)
//     let tx_id: Hash256 = apply_ctx.tx.common_fields.transaction_id;
//     // match write!(writer, "{:02X}", byte) {
//     // info!("Simulated tx_id from apply_ctx: {:02X}", tx_id);
//
//     let tx_id_size = 32u32;
//
//     // 4. Bounds Check (Matches NOT_IN_BOUNDS)
//     let memory_size = memory.size() * 65536; // Memory size is in pages (64KiB)
//     debug!("memory_size (KiB): {}", memory_size);
//
//     // Check if write_ptr + tx_id_size overflows or goes out of bounds
//     // Using checked_add to prevent overflow issues during check.
//     let end_ptr = match guest_write_ptr_u32.checked_add(tx_id_size) {
//         Some(end) => end,
//         None => {
//             println!("get_tx_hash_helper: Out of bounds (pointer + size overflow)");
//             return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
//         }
//     };
//
//     if end_ptr > memory_size {
//         println!(
//             "get_tx_hash_helper: Out of bounds (ptr {} + size {} > memory {})",
//             guest_write_ptr_u32, tx_id_size, memory_size
//         );
//         // Return OUT_OF_BOUNDS as i64
//         return Ok(vec![WasmValue::from_i64(OUT_OF_BOUNDS as i64)]);
//     }
//
//     // 6. Write to Memory (Matches WRITE_WASM_MEMORY_AND_RETURN)
//     debug!(
//         "get_tx_hash_helper: Writing {} bytes to pointer {}",
//         tx_id_size, guest_write_ptr_u32
//     );
//
//     let data: [u8; 32] = tx_id.0[..32].try_into().unwrap();
//     let result = memory.write(guest_write_ptr_u32 as usize, data);
//     match result {
//         Some(()) => {
//             // println!("Wasm memory write succeeded!");
//             // Proceed
//         }
//         None => {
//             eprintln!("Error: Wasm memory write failed. Check address and bounds.");
//             // Handle error
//             // return Err(HostFuncError::User( /* some error code */ ));
//         }
//     }
//
//     // 7. Result Handling (Matches C++ return logic)
//     // The C++ returns the number of bytes written (txID.size()) on success.
//     // let return_code = tx_id_size as i64;
//     // println!("get_tx_hash_helper: Success, wrote {return_code} bytes");
//     // TODO: What should WASM Get here? Hooks uses return codes, so we probably need something that's close to `OK`?
//     // Ok(vec![WasmValue::from_i64(return_code)])
//     Ok(vec![])
// }
