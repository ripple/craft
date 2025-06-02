use crate::decoding::{decode, AccountId, Decodable};
use crate::hashing::Hash256;
use crate::mock_data::{DataSource, Keylet, MockData};
use log::{debug, error, info};

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
    SlotsFull = -8,
    InvalidSlot = -9,
    LedgerObjNotFound = -10,
    DecodingError = -11,
    DataFieldTooLarge = -12,
    OutOfBound = -13,
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
        assert!(idx_fields.len() > 0);
        match self.data_source.get_field_value(source, idx_fields) {
            None => Self::fill_buf(None, buf_cap, Decodable::NOT),
            Some((last_field, field_result)) => Self::fill_buf(
                Some(field_result),
                buf_cap,
                Decodable::from_sfield(last_field),
            ),
        }
    }

    pub fn get_array_len(&self, source: DataSource, idx_fields: Vec<i32>) -> i32 {
        match self.data_source.get_array_len(source, idx_fields) {
            None => HostError::NoArray as i32,
            Some(len) => len as i32,
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
                            info!("is_u64::num: {}", num);
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
                                info!("bytes: {:?}", bytes);
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
                            // info!("bytes: {:?}", bytes);
                            if bytes.len() > buf_cap {
                                return (HostError::BufferTooSmall as i32, buf);
                            }
                            buf[..bytes.len()].copy_from_slice(&*bytes);
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
            None => (HostError::FieldNotFound as i32, buf),
        }
    }
}
