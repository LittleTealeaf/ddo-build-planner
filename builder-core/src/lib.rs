#![allow(dead_code)]
#![warn(
    missing_docs,
    clippy::inefficient_to_string,
    clippy::unwrap_used,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::equatable_if_let
)]

//! The goal of this crate is to implement all of the build logic, including attributes, breakdowns, enhancements, feats, items, and more

/// Encapsulates any character attribute into a single enum [Attribute]
///
/// [Attribute]: crate::attribute::Attribute
pub mod attribute;
pub mod bonus;
pub mod character;
pub mod compiler;
pub mod feat;
pub mod player_class;
pub mod utils;
