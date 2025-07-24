pub const ACCOUNT_ID_SIZE: usize = 20;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AccountID(pub [u8; ACCOUNT_ID_SIZE]);

impl From<[u8; ACCOUNT_ID_SIZE]> for AccountID {
    fn from(value: [u8; ACCOUNT_ID_SIZE]) -> Self {
        AccountID(value)
    }
}
