#![allow(dead_code)]
#![warn(
    missing_docs,
    clippy::inefficient_to_string,
    clippy::unwrap_used,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::equatable_if_let,
    clippy::if_then_some_else_none,
    clippy::if_not_else,
    clippy::implicit_clone,
    clippy::implicit_hasher
)]

//! The goal of this crate is to implement all of the build logic, including attributes, breakdowns, enhancements, feats, items, and more

/// Encapsulates any character attribute into a single enum [Attribute]
///
/// [Attribute]: crate::attribute::Attribute
pub mod attribute;
/// Represents bonuses, including the bonus type, source, and conditions associated.
pub mod bonus;
/// I don't know what quite yet..
pub mod character;
/// Performs calculations for breakdowns.
///
/// This is what takes in [`Bonus`] objects and calculates the resulting value for an
/// [`Attribute`].
pub mod compiler;
/// Represents Feats within the game
pub mod feat;
/// Represents Player Classes within the game
pub mod player_class;
/// Misc. Utility functions and Macros.
pub mod utils;
