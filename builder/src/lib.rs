//! The goal of this crate is to implement all of the build logic, including attributes, breakdowns, enhancements, feats, items, and more

pub mod attribute;
pub mod bonus;
pub mod breakdowns;
#[cfg(feature = "debug")]
pub mod debug;
pub mod equipment;
pub mod feat;
pub mod types;
