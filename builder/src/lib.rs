#![warn(missing_docs)]
#![allow(
    dead_code,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::module_name_repetitions
)]

//! The goal of this crate is to implement all of the build logic, including attributes, breakdowns, enhancements, feats, items, and more

pub mod attribute;
pub mod bonus;
pub mod compiler;
pub mod equipment;
pub mod feat;
pub mod player_class;
pub mod race;
pub mod types;
pub mod dice;
