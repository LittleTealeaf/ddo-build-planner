//! Describes the different race options that a player can be.

mod bonuses;
mod display;
mod feats;

pub use bonuses::*;
pub use display::*;
pub use feats::*;
use serde::{Deserialize, Serialize};

/// The different race options that the character can be.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Race {
    /// Dragonborn Race
    Dragonborn,
    /// Drow Race
    Drow,
    /// Dwarf Race
    Dwarf,
    /// Elf Race
    Elf,
    /// Gnome Race
    Gnome,
    /// Halfling Race
    Halfling,
    /// HalfElf Race
    HalfElf,
    /// HalfOrc Race
    HalfOrc,
    /// Human Race
    Human,
    /// Tiefling Race
    Tiefling,
    /// Warforged Race
    Warforged,
    /// WoodElf Race
    WoodElf,
    /// Aasimar Race
    Aasimar,
    /// Shifter Race
    Shifter,
    /// Tabaxi Race
    Tabaxi,
    /// Bladeforged Race
    Bladeforged,
    /// DeepGnome Race
    DeepGnome,
    /// Morninglord Race
    Morninglord,
    /// Purple Dragon Knight Race
    PurpleDragonKnight,
    /// Razorclaw Race
    Razorclaw,
    /// Scoundrel Race
    Scoundrel,
    /// Scourge Race
    Scourge,
    /// Shadarkai Race
    Shadarkai,
    /// Trailblazer Race
    Trailblazer,
}
