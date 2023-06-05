//! Describes the different race options that a player can be.

mod display;
mod bonuses;
mod feats;

use enum_map::Enum;

pub use display::*;
pub use bonuses::*;
pub use feats::*;


/// The different race options that the character can be.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
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
    Trailblazer
}
