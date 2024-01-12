//! This crate contains large datasets such as items.

#[macro_use]
mod util;

#[cfg(feature = "example")]
mod example;
#[cfg(feature = "example")]
pub use example::*;

mod set_bonuses;
pub use set_bonuses::*;
