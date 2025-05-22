pub const XRPL_ACCOUNT_ID_SIZE: usize = 20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AccountID(pub [u8; XRPL_ACCOUNT_ID_SIZE]);

impl From<[u8; 20]> for AccountID {
    fn from(value: [u8; 20]) -> Self {
        AccountID(value)
    }
}
