// #[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AccountID(pub [u8; 20]);
