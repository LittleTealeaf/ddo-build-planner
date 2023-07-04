#![warn(missing_docs, clippy::pedantic, clippy::nursery)]
#![allow(dead_code, clippy::must_use_candidate, clippy::too_many_lines)]

//! This crate contains large datasets such as items.

#[macro_use]
mod util;

#[cfg(feature = "example")]
mod example;
#[cfg(feature = "example")]
pub use example::*;
