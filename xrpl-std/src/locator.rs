const LOCATOR_BUFFER_SIZE: usize = 64;
const PATH_LEN_MAX: usize =  15;
const PATH_LEN_INDEX: usize = 3;
const PATH_START_INDEX: usize = 4;
pub enum DataSource {
    Tx,
    CurrentLedgerObj,
    KeyletLedgerObj(u8),
}
impl DataSource {
    pub fn to_i32(&self) -> i32 {
        match self {
            DataSource::Tx => 300,
            DataSource::CurrentLedgerObj => 400,
            DataSource::KeyletLedgerObj(n) => *n as i32,
        }
    }
}

pub struct Locator {
    buffer: [u8; LOCATOR_BUFFER_SIZE],
    cur_buffer_index: usize,
}

impl Locator {
    pub fn new(source: DataSource) -> Locator {
        let mut buffer: [u8; 64] = [0; 64];
        match source {
            DataSource::Tx => {
                buffer[0] = 0;
            }
            DataSource::CurrentLedgerObj => {
                buffer[0] = 1;
            }
            DataSource::KeyletLedgerObj(slot_num) => {
                buffer[0] = 2;
                buffer[1] = slot_num;
            }
        }
        Self {
            buffer,
            cur_buffer_index: PATH_START_INDEX,
        }
    }

    pub fn pack(&mut self, sfield_or_index: i32) -> bool {
        if self.cur_buffer_index + 4 > LOCATOR_BUFFER_SIZE {
            return false;
        }

        let value_bytes: [u8; 4] = sfield_or_index.to_le_bytes();
        for i in 0..value_bytes.len() {
            match self.buffer.get_mut(self.cur_buffer_index) {
                Some(b) => *b = value_bytes[i],
                None => return false,
            }
            self.cur_buffer_index += 1;
        }
        self.buffer[PATH_LEN_INDEX] += 1;
        true
    }

    pub fn get_addr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn num_packed_bytes(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn repack_last(&mut self, sfield_or_index: i32) -> bool {
        self.cur_buffer_index -= 4;
    
        let value_bytes: [u8; 4] = sfield_or_index.to_le_bytes();
        for i in 0..value_bytes.len() {
            self.buffer[self.cur_buffer_index] = value_bytes[i];
            self.cur_buffer_index += 1;
        }
        true
    }
}

impl Locator {
    pub fn from_bytes(buffer: [u8; LOCATOR_BUFFER_SIZE]) -> Locator {
        Locator {
            buffer,
            cur_buffer_index: PATH_START_INDEX,
        }
    }

    pub fn get_source(&self) -> Option<DataSource> {
        match self.buffer[0] {
            0 => Some(DataSource::Tx),
            1 => Some(DataSource::CurrentLedgerObj),
            2 => Some(DataSource::KeyletLedgerObj(self.buffer[1])),
            _ => None,
        }
    }

    pub fn unpack(&mut self) -> Option<i32> {
        let path_len = self.buffer[PATH_LEN_INDEX] as usize;
        if path_len > PATH_LEN_MAX {
            return None;
        }
        if path_len * 4 + PATH_START_INDEX <= self.cur_buffer_index {
            return None;
        }
        let mut bytes: [u8; 4] = [0u8; 4];
        bytes.copy_from_slice(&self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4]);
        self.cur_buffer_index += 4;
        Some(i32::from_le_bytes(bytes))
    }
}