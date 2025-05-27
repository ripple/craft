use crate::core::amount::xrp_amount::XrpAmount;

pub mod issued_currency_amount;
pub mod xrp_amount;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Copy)]
pub enum Amount {
    // IssuedCurrencyAmount(IssuedCurrencyAmount),
    // MptAmount(MptAmount)
    Xrp(XrpAmount),
}

impl Amount {
    pub fn is_xrp(&self) -> bool {
        match self {
            // Amount::IssuedCurrencyAmount => false,
            Amount::Xrp(_) => true,
            // _ => false,
        }
    }

    pub fn is_issued_currency(&self) -> bool {
        match self {
            Amount::Xrp(_) => false,
            // _ => false,
        }
    }
}

impl From<XrpAmount> for Amount {
    fn from(value: XrpAmount) -> Self {
        Amount::Xrp(value)
    }
}
