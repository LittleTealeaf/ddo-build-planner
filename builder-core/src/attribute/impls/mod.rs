//! Provides attribute implementations needed for different types, enums, or structs that are found elsewhere in the project.
//!
//! There may not be entries in this module, and that is to be expected as they only add implementations to pre-existing items.
mod race;
mod player_class;

pub use race::*;
pub use player_class::*;
