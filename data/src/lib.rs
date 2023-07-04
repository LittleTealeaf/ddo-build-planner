#![allow(dead_code, clippy::must_use_candidate)]
#![warn(missing_docs, clippy::pedantic, clippy::nursery)]

//! This crate contains large datasets such as items.

#[macro_use]
mod util;

#[cfg(feature = "example")]
mod example;
#[cfg(feature = "example")]
pub use example::*;
