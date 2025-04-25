#![no_std]
#![no_main]

pub mod core;
mod field;
mod mocks;
pub mod model;
pub mod sfield;
pub mod string;
pub mod util;

// #[cfg(target_arch = "wasm32")]
pub mod host {
    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        /// Log a string to std_out using the host for actual emission.
        pub fn log(str_ptr: *const u8, len: usize);

        // pub fn getLedgerSqn() -> i32;
        pub fn getCurrentTxField(sfield: i32) -> i32;
        // pub fn print(str_ptr: i32, str_len: i32);

        pub fn add(a: i32, b: i32) -> i32;

        // Obtain the specified account's current IOU balance.
        // pub fn getAccountBalanceSTAmount() -> u64;

        // we can add buff_len if needed
        pub fn getCurrentTxField_Peng(buff_ptr: *mut u8, packed: usize) -> i32;
    }
}

const BUFF_SIZE: usize = 4096;
static mut BUFF: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
static mut BUFF_PTR: *mut u8 = &mut unsafe { BUFF }[0] as *mut u8;

enum LocatorItemType {
    Sfield,
    ArrayIndex,
}

pub struct LocatorPacker {
    cur: usize,
    // packed: u8,
}

impl LocatorPacker {
    pub fn new() -> Self {
        Self {
            cur: 0,
            // packed: 0,
        }
    }

    fn pack_type(&mut self, t: LocatorItemType) {
        unsafe {
            *(BUFF_PTR.add(self.cur)) = match t {
                LocatorItemType::Sfield => 0u8,
                LocatorItemType::ArrayIndex => 1u8,
            };
            self.cur += 1;
        }
    }

    fn pack_u32(&mut self, t: LocatorItemType, v: u32) -> bool {
        unsafe {
            if self.cur + 4 > BUFF_SIZE {
                return false;
            }
            self.pack_type(t);
            let b = v.to_le_bytes();
            for i in 0..b.len() {
                *(BUFF_PTR.add(self.cur)) = b[i];
                self.cur += 1;
            }
            return true;
        }
    }

    pub fn pack_sfield(&mut self, sfield: i32) -> bool {
        self.pack_u32(LocatorItemType::Sfield, sfield as u32)
    }

    pub fn pack_array_index(&mut self, array_index: i32) -> bool {
        self.pack_u32(LocatorItemType::ArrayIndex, array_index as u32)
    }

    pub fn get_addr(&self) -> *mut u8 {
        unsafe {
            BUFF_PTR
        }
    }
    pub fn bytes_packed(&self) -> usize {
        unsafe {
            self.cur
        }
    }
}


