use crate::decoding::{AccountId, Decodable, decode, decode_amount_json, decode_issue_json};
use crate::hashing::Hash256;
use crate::mock_data::{DataSource, Keylet, MockData};
use std::ffi::c_void;

const LOCATOR_BUFFER_SIZE: usize = 64;
const NUM_SLOTS: usize = 256;
pub const XRPL_CONTRACT_DATA_SIZE: usize = 4096;

#[allow(dead_code)]
pub enum HostError {
    InternalError = -1,
    FieldNotFound = -2,
    BufferTooSmall = -3,
    NoArray = -4,
    NotLeafField = -5,
    LocatorMalformed = -6,
    SlotOutRange = -7,
    NoFreeSlots = -8,
    EmptySlot = -9,
    LedgerObjNotFound = -10,
    DecodingError = -11,
    DataFieldTooLarge = -12,
    PointerOutOfBound = -13, // WAMR VM checks, so we don't need to
    NoMemoryExported = -14,  // We don't explicitly call WAMR memory functions.
    InvalidParams = -15,
    InvalidAccount = -16,
    InvalidField = -17,
    IndexOutOfBounds = -18,
}

impl From<i64> for HostError {
    fn from(value: i64) -> Self {
        match value {
            -1 => HostError::InternalError,
            -2 => HostError::FieldNotFound,
            -3 => HostError::BufferTooSmall,
            -4 => HostError::NoArray,
            -5 => HostError::NotLeafField,
            -6 => HostError::LocatorMalformed,
            -7 => HostError::SlotOutRange,
            -8 => HostError::NoFreeSlots,
            -9 => HostError::EmptySlot,
            -10 => HostError::LedgerObjNotFound,
            -11 => HostError::DecodingError,
            -12 => HostError::DataFieldTooLarge,
            -13 => HostError::PointerOutOfBound,
            -14 => HostError::NoMemoryExported,
            -15 => HostError::InvalidParams,
            -16 => HostError::InvalidAccount,
            -17 => HostError::InvalidField,
            -18 => HostError::IndexOutOfBounds,
            _ => HostError::InternalError, // Default to InternalError for unknown error codes
        }
    }
}

/// Converts an error code to its string representation.
///
/// # Arguments
///
/// * `code` - An integer representing the error code
///
/// # Returns
///
/// Returns a string slice representing the name of the error code constant and its integer value.
/// Returns "UNKNOWN_ERROR (code)" if the error code is not recognized.
/// ```
pub fn error_code_to_string(code: i64) -> &'static str {
    // Convert the code to a HostError
    let host_error: HostError = code.into();

    // Match on the HostError
    match host_error {
        HostError::InternalError => "INTERNAL_ERROR (-1)",
        HostError::FieldNotFound => "FIELD_NOT_FOUND (-2)",
        HostError::BufferTooSmall => "BUFFER_TOO_SMALL (-3)",
        HostError::NoArray => "NO_ARRAY (-4)",
        HostError::NotLeafField => "NOT_LEAF_FIELD (-5)",
        HostError::LocatorMalformed => "LOCATOR_MALFORMED (-6)",
        HostError::SlotOutRange => "SLOT_OUT_RANGE (-7)",
        HostError::NoFreeSlots => "SLOTS_FULL (-8)",
        HostError::EmptySlot => "EMPTY_SLOT (-9)",
        HostError::LedgerObjNotFound => "LEDGER_OBJ_NOT_FOUND (-10)",
        HostError::DecodingError => "DECODING_ERROR (-11)",
        HostError::DataFieldTooLarge => "DATA_FIELD_TOO_LARGE (-12)",
        HostError::PointerOutOfBound => "POINTER_OUT_OF_BOUND (-13)",
        HostError::NoMemoryExported => "NO_MEMORY_EXPORTED (-14)",
        HostError::InvalidParams => "INVALID_PARAMS (-15)",
        HostError::InvalidAccount => "INVALID_ACCOUNT (-16)",
        HostError::InvalidField => "INVALID_FIELD (-17)",
        HostError::IndexOutOfBounds => "INDEX_OUT_OF_BOUNDS (-18)",
    }
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
                return HostError::NoFreeSlots as i32;
            }
            slot = self.next_slot;
            self.next_slot += 1;
        } else if slot >= NUM_SLOTS {
            return HostError::SlotOutRange as i32;
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
        assert!(!idx_fields.is_empty());
        let (last_sfield, field_result) = match self.data_source.get_field_value(source, idx_fields)
        {
            Ok(v) => v,
            Err(e) => return (e as i32, vec![]),
        };

        Self::fill_buf(
            Some(field_result),
            buf_cap,
            Decodable::from_sfield(last_sfield),
        )
    }

    pub fn get_array_len(&self, source: DataSource, idx_fields: Vec<i32>) -> i32 {
        let (_, value) = match self.data_source.get_field_value(source, idx_fields) {
            Ok(v) => v,
            Err(e) => return e as i32,
        };
        if value.is_array() {
            value.as_array().unwrap().len() as i32
        } else {
            HostError::NoArray as i32
        }
    }

    pub fn get_ledger_sqn(&self, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_ledger_sqn();
        Self::fill_buf(field_result, buf_cap, Decodable::NOT)
    }

    pub fn get_parent_ledger_time(&self, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_parent_ledger_time();
        Self::fill_buf(field_result, buf_cap, Decodable::NOT)
    }

    pub fn get_parent_ledger_hash(&self, buf_cap: usize) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_parent_ledger_hash();
        Self::fill_buf(field_result, buf_cap, Decodable::UINT256)
    }

    pub fn get_nft_uri(
        &self,
        nft_id: &Hash256,
        account_id: &AccountId,
        buf_cap: usize,
    ) -> (i32, Vec<u8>) {
        let field_result = self.data_source.get_nft_uri(nft_id, account_id);
        Self::fill_buf(field_result, buf_cap, Decodable::AS_IS)
    }

    pub fn set_current_ledger_obj_data(&mut self, data: Vec<u8>) {
        self.data_source.set_current_ledger_obj_data(data);
    }

    fn fill_buf(
        field_result: Option<&serde_json::Value>,
        buf_cap: usize,
        decodable: Decodable,
    ) -> (i32, Vec<u8>) {
        let mut buf = vec![0u8; buf_cap];
        match field_result {
            Some(value) => {
                if decodable == Decodable::AMOUNT {
                    match decode_amount_json(value.clone()) {
                        None => (HostError::DecodingError as i32, buf),
                        Some(bytes) => {
                            if bytes.len() > buf_cap {
                                return (HostError::BufferTooSmall as i32, buf);
                            }
                            buf[..bytes.len()].copy_from_slice(&bytes);
                            (bytes.len() as i32, buf)
                        }
                    }
                } else if decodable == Decodable::ISSUE {
                    match decode_issue_json(value.clone()) {
                        None => (HostError::DecodingError as i32, buf),
                        Some(bytes) => {
                            if bytes.len() > buf_cap {
                                return (HostError::BufferTooSmall as i32, buf);
                            }
                            buf[..bytes.len()].copy_from_slice(&bytes);
                            (bytes.len() as i32, buf)
                        }
                    }
                } else {
                    match value {
                        serde_json::Value::Number(n) => {
                            if n.is_i64() {
                                let num = n.as_i64().unwrap();
                                if buf_cap == 4 {
                                    // Safe cast to i32
                                    if num > u32::MAX as i64 || num < u32::MIN as i64 {
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
                            } else if n.is_f64() {
                                let s = n.as_f64().unwrap().to_string();
                                let bytes = s.as_bytes();
                                if bytes.len() > buf_cap {
                                    return (HostError::BufferTooSmall as i32, buf);
                                }
                                buf[..bytes.len()].copy_from_slice(bytes);
                                return (bytes.len() as i32, buf);
                            } else {
                                return (HostError::InternalError as i32, buf);
                            }
                        }
                        serde_json::Value::String(s) => match decode(s, decodable) {
                            None => (HostError::DecodingError as i32, buf),
                            Some(bytes) => {
                                if bytes.len() > buf_cap {
                                    return (HostError::BufferTooSmall as i32, buf);
                                }
                                buf[..bytes.len()].copy_from_slice(&bytes);
                                (bytes.len() as i32, buf)
                            }
                        },
                        serde_json::Value::Bool(b) => {
                            if buf_cap == 0 {
                                return (HostError::BufferTooSmall as i32, buf);
                            }
                            buf[0] = if *b { 1 } else { 0 };
                            (1, buf)
                        }
                        // be explicit about the cases we don't support
                        serde_json::Value::Null => (HostError::NotLeafField as i32, buf),
                        serde_json::Value::Array(_) => (HostError::NotLeafField as i32, buf),
                        serde_json::Value::Object(_) => (HostError::NotLeafField as i32, buf),
                    }
                }
            }
            None => (HostError::FieldNotFound as i32, buf),
        }
    }
    #[allow(unused)]
    pub fn as_ptr(&mut self) -> *mut c_void {
        self as *mut _ as *mut c_void
    }
}
