const LOCATOR_BUFFER_SIZE: usize = 64;

pub struct LocatorPacker {
    buffer: [u8; LOCATOR_BUFFER_SIZE],
    cur_buffer_index: usize,
}

impl LocatorPacker {
    pub fn new() -> LocatorPacker {
        Self {
            buffer: [0; 64],
            cur_buffer_index: 0,
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
            match self.buffer.get_mut(self.cur_buffer_index) {
                Some(b) => *b = value_bytes[i],
                None => return false,
            }
            self.cur_buffer_index += 1;
        }
        true
    }
}

//host side
pub struct LocatorUnpacker {
    buffer: [u8; LOCATOR_BUFFER_SIZE],
    cur_buffer_index: usize,
    packed_bytes: usize,
}

impl LocatorUnpacker {
    pub fn from_bytes(buffer: [u8; LOCATOR_BUFFER_SIZE], packed_bytes: usize) -> Option<LocatorUnpacker> {
        if packed_bytes > LOCATOR_BUFFER_SIZE {
            None
        }else {
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
