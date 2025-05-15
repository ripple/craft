use crate::mock_data::{DataSource, Hash256, Keylet, MockData};
use sha2::{Digest, Sha512};
use wasmedge_sdk::error::{CoreError, CoreExecutionError};
use wasmedge_sdk::{CallingFrame, Instance, WasmValue};

const LOCATOR_BUFFER_SIZE: usize = 64;
const NUM_SLOTS: usize = 256;

type AccountId = Vec<u8>; //TODO size

pub enum HostError {
    InternalError = -1,
    FieldNotFound = -2,
    BufferTooSmall = -3,
    NoArray = -4,
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
}

impl LocatorUnpacker {
    pub fn from_bytes(buffer: Vec<u8>) -> Option<LocatorUnpacker> {
        let packed_bytes: usize = buffer.len();
        if packed_bytes > LOCATOR_BUFFER_SIZE || packed_bytes == 0 || packed_bytes % 4 != 0 {
            None
        } else {
            Some(LocatorUnpacker {
                buffer,
                cur_buffer_index: 0,
            })
        }
    }

    pub fn unpack(&mut self) -> Option<i32> {
        if self.cur_buffer_index + 4 > self.buffer.len() {
            return None;
        }
        let mut bytes: [u8; 4] = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4]);
        self.cur_buffer_index += 4;
        Some(i32::from_le_bytes(bytes))
    }
}

pub fn unpack_locator(buffer: Vec<u8>) -> Result<Vec<i32>, HostError> {
    let mut unpacker = LocatorUnpacker::from_bytes(buffer).ok_or(HostError::LocatorMalformed)?;

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

    pub fn get_field_value(
        &self,
        source: DataSource,
        idx_fields: Vec<i32>,
        buf_cap: usize,
    ) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_field_value(source, idx_fields);
        Self::fill_buf(field_result, buf_cap)
    }

    pub fn get_array_len(&self, source: DataSource, idx_fields: Vec<i32>) -> i32 {
        match self.data_source.get_array_len(source, idx_fields) {
            None => HostError::NoArray as i32,
            Some(len) => len as i32,
        }
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

    pub fn set_current_ledger_obj_data(&mut self, data: Vec<u8>) {
        self.data_source.set_current_ledger_obj_data(data);
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
                    _ => (HostError::InternalError as i32, buf),
                }
            }
            None => (HostError::FieldNotFound as i32, buf),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerNameSpace {
    Account = b'a' as u16,
    DirNode = b'd' as u16,
    TrustLine = b'r' as u16,
    Offer = b'o' as u16,
    OwnerDir = b'O' as u16,
    BookDir = b'B' as u16,
    SkipList = b's' as u16,
    Escrow = b'u' as u16,
    Amendments = b'f' as u16,
    FeeSettings = b'e' as u16,
    Ticket = b'T' as u16,
    SignerList = b'S' as u16,
    XrpPaymentChannel = b'x' as u16,
    Check = b'C' as u16,
    DepositPreauth = b'p' as u16,
    DepositPreauthCredentials = b'P' as u16,
    NegativeUnl = b'N' as u16,
    NftokenOffer = b'q' as u16,
    NftokenBuyOffers = b'h' as u16,
    NftokenSellOffers = b'i' as u16,
    Amm = b'A' as u16,
    Bridge = b'H' as u16,
    XchainClaimId = b'Q' as u16,
    XchainCreateAccountClaimId = b'K' as u16,
    Did = b'I' as u16,
    Oracle = b'R' as u16,
    MptokenIssuance = b'~' as u16,
    Mptoken = b't' as u16,
    Credential = b'D' as u16,
    PermissionedDomain = b'm' as u16,

    #[deprecated]
    Contract = b'c' as u16,
    #[deprecated]
    Generator = b'g' as u16,
    #[deprecated]
    Nickname = b'n' as u16,
}

pub fn sha512_half(data: &[u8]) -> Hash256 {
    let mut hasher = Sha512::new();
    hasher.update(&data);
    let result = hasher.finalize();
    result[..32].to_vec()
}

pub fn index_hash(space: LedgerNameSpace, args: &[u8]) -> Hash256 {
    let mut data = Vec::with_capacity(2 + args.len());
    data.extend_from_slice(&(space as u16).to_le_bytes());
    data.extend_from_slice(args);
    sha512_half(&data)
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
    let buffer = memory
        .get_data(in_buf_ptr as u32, in_buf_len as u32)
        .map_err(|e| {
            eprintln!(
                "get_tx_hash_helper: Error: Failed to get memory data: {}",
                e
            );
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

fn get_locator_data(
    in_buf_ptr: i32,
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
        memory
            .set_data(&data_to_write, out_buf_ptr as u32)
            .map_err(|e| {
                eprintln!(
                    "get_tx_hash_helper: Error: Failed to set memory data: {}",
                    e
                );
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
    let dp_res = _data_provider.get_field_value(
        DataSource::CurrentLedgerObj,
        vec![field],
        out_buf_cap as usize,
    );
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
    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };
    let dp_res = _data_provider.get_field_value(
        DataSource::KeyletLedgerObj(keylet),
        vec![field],
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_nested_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };

    let dp_res = _data_provider.get_field_value(DataSource::Tx, idx_fields, out_buf_cap as usize);
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_current_ledger_obj_nested_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };

    let dp_res = _data_provider.get_field_value(
        DataSource::CurrentLedgerObj,
        idx_fields,
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_ledger_obj_nested_field(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let in_buf_ptr: i32 = _inputs[1].to_i32();
    let in_buf_len: i32 = _inputs[2].to_i32();
    let out_buf_ptr: i32 = _inputs[3].to_i32();
    let out_buf_cap: i32 = _inputs[4].to_i32();
    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };

    let dp_res = _data_provider.get_field_value(
        DataSource::KeyletLedgerObj(keylet),
        idx_fields,
        out_buf_cap as usize,
    );
    set_data(dp_res.0, out_buf_ptr, dp_res.1, _caller)?;
    Ok(vec![WasmValue::from_i32(dp_res.0)])
}

pub fn get_tx_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::Tx, vec![field]),
    )])
}

pub fn get_current_ledger_obj_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let field: i32 = _inputs[0].to_i32();
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::CurrentLedgerObj, vec![field]),
    )])
}

pub fn get_ledger_obj_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let field: i32 = _inputs[1].to_i32();

    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };
    Ok(vec![WasmValue::from_i32(_data_provider.get_array_len(
        DataSource::KeyletLedgerObj(keylet),
        vec![field],
    ))])
}

pub fn get_tx_nested_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::Tx, idx_fields),
    )])
}

pub fn get_current_ledger_obj_nested_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };
    Ok(vec![WasmValue::from_i32(
        _data_provider.get_array_len(DataSource::CurrentLedgerObj, idx_fields),
    )])
}

pub fn get_ledger_obj_nested_array_len(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let slot: i32 = _inputs[0].to_i32();
    let in_buf_ptr: i32 = _inputs[1].to_i32();
    let in_buf_len: i32 = _inputs[2].to_i32();
    let keylet = match _data_provider.slot_get(slot as usize) {
        None => return Ok(vec![WasmValue::from_i32(HostError::SlotOutRange as i32)]),
        Some(key) => key.clone(),
    };

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let idx_fields: Vec<i32> = match unpack_locator(data) {
        Ok(fields) => fields,
        Err(host_err) => return Ok(vec![WasmValue::from_i32(host_err as i32)]),
    };
    Ok(vec![WasmValue::from_i32(_data_provider.get_array_len(
        DataSource::KeyletLedgerObj(keylet),
        idx_fields,
    ))])
}

pub fn update_data(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    _data_provider.set_current_ledger_obj_data(data);
    Ok(vec![])
}

pub fn compute_sha512_half(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let hash_half = sha512_half(&data);
    if hash_half.len() > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }

    set_data(hash_half.len() as i32, out_buf_ptr, hash_half, _caller)?;
    Ok(vec![WasmValue::from_i32(32)])
}

pub fn account_keylet(
    _data_provider: &mut DataProvider,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _inputs: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    let in_buf_ptr: i32 = _inputs[0].to_i32();
    let in_buf_len: i32 = _inputs[1].to_i32();
    let out_buf_ptr: i32 = _inputs[2].to_i32();
    let out_buf_cap: i32 = _inputs[3].to_i32();

    let data = get_data(in_buf_ptr, in_buf_len, _caller)?;
    let keylet_hash = index_hash(LedgerNameSpace::Account, &data);
    if keylet_hash.len() > out_buf_cap as usize {
        return Ok(vec![WasmValue::from_i32(HostError::BufferTooSmall as i32)]);
    }

    set_data(keylet_hash.len() as i32, out_buf_ptr, keylet_hash, _caller)?;
    Ok(vec![WasmValue::from_i32(32)])
}
