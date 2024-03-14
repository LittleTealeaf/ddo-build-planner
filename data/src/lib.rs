//! This crate contains large datasets such as items.

#[macro_use]
mod util;

/// Data Parsing Error
pub type ParseError = SpannedError;

#[cfg(feature = "example")]
mod example;
#[cfg(feature = "example")]
pub use example::*;

mod item_sets;
use ron::error::SpannedError;
pub use item_sets::*;
