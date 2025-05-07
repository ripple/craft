// #[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AccountID(pub [u8; 20]);

// #[repr(C)]
// pub struct Result {
//     error_code: i32,
//     result: i64,
// }
