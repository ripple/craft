/// TODO: Cleanup docs!
/// TODO: Add unit tests for packing!
// Locator2 (with nested values): `[1u8]...[1u8][0u32]...[1u8][0u32]...[1u8][0u32]`
//
// * Byte0: slot_num
// * For each Locator
// * Byte1 (Locator Type)
// *   -- 0 for `sfield`
// *   -- 1 for `array index`
// TODO: Could we remove this byte and pick 0 - 1024 as an index, and everything else as an sField
// Alt: Reserve 8 MSB of the field code as a field-picker (or fewer bytes)? Risky!
// * Byte2 - 6: `field_code` // e.g., "sfAccount" // 524289
const LOCATOR_BUFFER_SIZE: u8 = 64;

/// A Locator may only pack this many levels deep in an object hierarchy (inclusive of first field)
const MAX_DEPTH:u8 = 12; // 1 byte for slot; 5 bytes for each packed object.

/// A Locator allows a WASM developer located any field in any object (even nested fields) by
/// specifying a `slot_num` (1 byte); a `locator_field_type` (1 byte); then one of an `sfield` (4
/// bytes) or an `index` (4 bytes).  
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Locator {
    // First packed value is 6 bytes; All nested/packed values are 5 bytes; so 64 bytes allows
    // 12 nested levels of access.
    buffer: [u8; LOCATOR_BUFFER_SIZE as usize],

    /// An index into `buffer` where the next packing operation can be stored.
    cur_buffer_index: u8,
}

impl Locator {
    // TODO: Add a LocatorType so that a slot is different from the type that goes into a slot.
    // TODO: Peng calls this a Datasource type.

    /// Create a new Locator using an unsigned 8-bit number. Valid slots are 0 to 255.
    pub fn new(slot_num: u8) -> Locator {
        let mut buffer: [u8; 64] = [0; 64];
        buffer[0] = slot_num;
        Self {
            buffer,
            cur_buffer_index: 1,
        }
    }

    fn pack_locator_type(&mut self, t: LocatorItemType) {
        self.buffer[self.cur_buffer_index as usize] = match t {
            LocatorItemType::SField => 0u8,
            LocatorItemType::ArrayIndex => 1u8,
        };
        self.cur_buffer_index += 1;
    }

    fn pack_u32(&mut self, locator_item_type: LocatorItemType, value: u32) -> bool {
        if self.cur_buffer_index + 4 > LOCATOR_BUFFER_SIZE {
            return false;
        }

        self.pack_locator_type(locator_item_type);

        let value_bytes: [u8; 4] = value.to_le_bytes();
        for i in 0..value_bytes.len() {
            self.buffer[self.cur_buffer_index as usize] = value_bytes[i];
            self.cur_buffer_index += 1;
        }
        true
    }

    pub fn pack_sfield(&mut self, field_code: i32) -> bool {
        self.pack_u32(LocatorItemType::SField, field_code as u32)
    }

    pub fn pack_array_index(&mut self, array_index: i32) -> bool {
        self.pack_u32(LocatorItemType::ArrayIndex, array_index as u32)
    }

    pub fn get_addr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn num_packed_bytes(&self) -> u8 {
        self.cur_buffer_index
    }
}

enum LocatorItemType {
    SField = 0,
    ArrayIndex = 1,
}
