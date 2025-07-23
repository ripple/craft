//TODO add docs after discussing the interface
//Note that Craft currently does not honor the rounding modes
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TO_NEAREST: i32 = 0;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TOWARDS_ZERO: i32 = 1;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_DOWNWARD: i32 = 2;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_UPWARD: i32 = 3;

// pub enum RippledRoundingModes{
//     ToNearest = 0,
//     TowardsZero = 1,
//     DOWNWARD = 2,
//     UPWARD = 3
// }

#[allow(unused)]
#[link(wasm_import_module = "host_lib")]
unsafe extern "C" {
    pub fn get_ledger_sqn(out_buff_ptr: i32, out_buff_len: i32) -> i32;
}
