use crate::core::types::amount::float::XrplFloat;

#[derive(Copy, Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub struct IouAmount(pub XrplFloat);

impl IouAmount {
    pub fn new(iou_amount: XrplFloat) -> Self {
        Self(iou_amount)
    }
}
impl Default for IouAmount {
    fn default() -> Self {
        Self(XrplFloat(0))
    }
}

impl From<IouAmount> for [u8; 8] {
    fn from(value: IouAmount) -> Self {
        let value: u64 = value.0.0;
        value.to_le_bytes()
    }
}
