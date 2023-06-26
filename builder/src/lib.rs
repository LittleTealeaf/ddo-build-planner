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
    clippy::implicit_hasher,
    clippy::dbg_macro,
    clippy::default_trait_access,
    clippy::empty_line_after_outer_attr,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::get_unwrap,
    clippy::large_types_passed_by_value,
    clippy::manual_ok_or,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

//! The goal of this crate is to implement all of the build logic, including attributes, breakdowns, enhancements, feats, items, and more

pub mod attribute;
pub mod bonus;
pub mod compiler;
pub mod feat;
pub mod equipment;
pub mod player_class;
pub mod race;
