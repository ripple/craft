//! # Amount Types Module
//!
//! This module provides comprehensive support for handling different types of amounts
//! within the XRP Ledger (XRPL) ecosystem. It defines core amount types and provides
//! a unified interface for working with XRP, Issued Currencies (IOUs), and Multi-Purpose
//! Tokens (MPTs).
//!
//! # Examples
//!
//! ```rust
//! // use crate::core::types::amount::{Amount, XrpAmount, IouAmount, MptAmount};
//!
//! // Create different amount types
//!
//! use xrpl_std::core::types::amount::{Amount, AmountTypeTrait};
//! use xrpl_std::core::types::amount::float::XrplFloat;
//! use xrpl_std::core::types::amount::issued_currency_amount::IouAmount;
//! use xrpl_std::core::types::amount::mpt_amount::MptAmount;
//! use xrpl_std::core::types::amount::xrp_amount::XrpAmount;
//!
//! let xrp_amount = Amount::Xrp(XrpAmount::new(1000000)); // 1 XRP in drops
//! let iou_amount = Amount::Iou(IouAmount::new(XrplFloat(0)));
//! let mpt_amount = Amount::Mpt(MptAmount::new(500000));
//!
//! // Check amount types
//! assert!(xrp_amount.is_xrp());
//! assert!(!xrp_amount.is_iou());
//! assert!(!xrp_amount.is_mpt());
//! ```

use crate::core::types::amount::issued_currency_amount::IouAmount;
use crate::core::types::amount::mpt_amount::MptAmount;
use crate::core::types::amount::xrp_amount::XrpAmount;

pub mod float;
pub mod issued_currency_amount;
pub mod mpt_amount;
pub mod xrp_amount;

#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd, Copy)]
pub enum Amount {
    Iou(IouAmount),
    Mpt(MptAmount),
    Xrp(XrpAmount),
}

/// Trait providing type checking functionality for XRPL amount types.
///
/// This trait enables runtime type identification for different amount variants
/// in the XRP Ledger ecosystem. It provides a consistent interface for determining
/// whether an amount represents XRP, an Issued Currency (IOU), or a Multi-Purpose Token (MPT).
///
/// # Purpose
///
/// The trait allows for polymorphic behavior when working with different amount types,
/// enabling code to make decisions based on the specific type of amount without
/// requiring explicit pattern matching or type casting.
///
/// # Default Behavior
///
/// The default implementations assume the type is an `Amount` enum and use pattern
/// matching to determine the variant. Individual amount types should override these
/// methods to return their specific type information more efficiently.
///
/// # Examples
///
/// ## Basic Type Checking
/// ```rust
/// use xrpl_std::core::types::amount::{Amount, AmountTypeTrait};
/// use xrpl_std::core::types::amount::mpt_amount::MptAmount;
/// use xrpl_std::core::types::amount::xrp_amount::XrpAmount;
///
/// let xrp_amount = Amount::Xrp(XrpAmount::new(1000000));
/// let mpt_amount = Amount::Mpt(MptAmount::new(500000));
///
/// assert!(xrp_amount.is_xrp());
/// assert!(!xrp_amount.is_mpt());
/// assert!(mpt_amount.is_mpt());
/// assert!(!mpt_amount.is_xrp());
/// ```
///
/// ## Polymorphic Processing
/// ```rust
/// use xrpl_std::core::types::amount::AmountTypeTrait;
///
/// fn process_amount<T: AmountTypeTrait>(amount: &T) {
///     if amount.is_xrp() {
///         println!("Processing XRP amount");
///     } else if amount.is_iou() {
///         println!("Processing issued currency amount");
///     } else if amount.is_mpt() {
///         println!("Processing multi-purpose token amount");
///     }
/// }
/// ```
///
/// ## Conditional Logic
/// ```rust
/// use xrpl_std::core::types::amount::AmountTypeTrait;
///
/// fn calculate_fees<T: AmountTypeTrait>(amount: &T) -> u64 {
///     match (amount.is_xrp(), amount.is_iou(), amount.is_mpt()) {
///         (true, false, false) => 10,    // XRP fee
///         (false, true, false) => 15,    // IOU fee
///         (false, false, true) => 12,    // MPT fee
///         _ => panic!("Invalid amount type combination"),
///     }
/// }
/// ```
pub trait AmountTypeTrait {
    /// Returns `true` if this amount represents native XRP.
    ///
    /// # Examples
    /// ```rust
    /// use xrpl_std::core::types::amount::{Amount, AmountTypeTrait};
    /// use xrpl_std::core::types::amount::mpt_amount::MptAmount;
    /// use xrpl_std::core::types::amount::xrp_amount::XrpAmount;
    ///
    /// let xrp_amount = Amount::Xrp(XrpAmount::new(1000000));
    /// assert!(xrp_amount.is_xrp());
    ///
    /// let mpt_amount = Amount::Mpt(MptAmount::new(500000));
    /// assert!(!mpt_amount.is_xrp());
    /// ```
    fn is_xrp(&self) -> bool {
        false
    }

    /// Returns `true` if this amount represents an Issued Currency (IOU).
    ///
    /// # Examples
    /// ```rust
    /// use xrpl_std::core::types::amount::{Amount, AmountTypeTrait};
    /// use xrpl_std::core::types::amount::float::XrplFloat;
    /// use xrpl_std::core::types::amount::issued_currency_amount::IouAmount;
    /// use xrpl_std::core::types::amount::xrp_amount::XrpAmount;
    ///
    /// let iou_amount = Amount::Iou(IouAmount::new(XrplFloat(0u64)));
    /// assert!(iou_amount.is_iou());
    ///
    /// let xrp_amount = Amount::Xrp(XrpAmount::new(1000000));
    /// assert!(!xrp_amount.is_iou());
    /// ```
    fn is_iou(&self) -> bool {
        false
    }

    /// Returns `true` if this amount represents a Multi-Purpose Token (MPT).
    ///
    /// # Examples
    /// ```rust
    /// use xrpl_std::core::types::amount::{Amount, AmountTypeTrait};
    /// use xrpl_std::core::types::amount::mpt_amount::MptAmount;
    /// use xrpl_std::core::types::amount::xrp_amount::XrpAmount;
    ///
    /// let mpt_amount = Amount::Mpt(MptAmount::new(500000));
    /// assert!(mpt_amount.is_mpt());
    ///
    /// let xrp_amount = Amount::Xrp(XrpAmount::new(1000000));
    /// assert!(!xrp_amount.is_mpt());
    /// ```
    fn is_mpt(&self) -> bool {
        false
    }
}

impl AmountTypeTrait for Amount {
    fn is_xrp(&self) -> bool {
        matches!(self, Self::Xrp(_))
    }

    fn is_iou(&self) -> bool {
        matches!(self, Self::Iou(_))
    }

    fn is_mpt(&self) -> bool {
        matches!(self, Self::Mpt(_))
    }
}
impl AmountTypeTrait for XrpAmount {
    fn is_xrp(&self) -> bool {
        true
    }
}
impl AmountTypeTrait for IouAmount {
    fn is_iou(&self) -> bool {
        true
    }
}
impl AmountTypeTrait for MptAmount {
    fn is_mpt(&self) -> bool {
        true
    }
}

impl From<XrpAmount> for Amount {
    fn from(value: XrpAmount) -> Self {
        Amount::Xrp(value)
    }
}

impl From<IouAmount> for Amount {
    fn from(value: IouAmount) -> Self {
        Amount::Iou(value)
    }
}

impl From<MptAmount> for Amount {
    fn from(value: MptAmount) -> Self {
        Amount::Mpt(value)
    }
}
