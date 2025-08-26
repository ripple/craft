#![doc = include_str!("../README.md")]

//! # Craft - XRPL Smart Contract Development Toolkit
//!
//! Craft is a toolkit for developing, testing, and deploying
//! XRPL smart contracts written in WebAssembly.
//!
//! See the [README](index.html) for complete documentation and getting started guide.

// Re-export modules for documentation
pub mod commands;
pub mod config;
pub mod docker;
pub mod utils;

/// Additional guides and how-tos
#[cfg(doc)]
pub mod guides {
    /// XRPL Field Access and Locators guide
    #[doc = include_str!("../docs/FIELD_ACCESS.md")]
    pub mod field_access {}
}