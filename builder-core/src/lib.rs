#![allow(dead_code)]
#![warn(missing_docs)]
//! The goal of this crate is to implement all of the build logic, including attributes, breakdowns, enhancements, feats, items, and more

/// Encapsulates any character attribute into a single enum [Attribute]
///
/// [Attribute]: crate::attribute::Attribute
pub mod attribute;
pub mod bonus;
pub mod breakdown;
pub mod character;
pub mod feat;
pub mod player_class;
pub mod utils;
pub mod compiler;
