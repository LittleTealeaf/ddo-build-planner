//! Contains player race
public_modules!(bonuses);

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utils::public_modules;

use crate::attribute::{Attribute, ToAttribute};

use super::flag::Flag;

/// The different race options that the character can be.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
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

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dragonborn => write!(f, "Dragonborn"),
            Self::Drow => write!(f, "Drow"),
            Self::Dwarf => write!(f, "Dwarf"),
            Self::Elf => write!(f, "Elf"),
            Self::Gnome => write!(f, "Gnome"),
            Self::Halfling => write!(f, "Halfling"),
            Self::HalfElf => write!(f, "Half Elf"),
            Self::HalfOrc => write!(f, "Half Orc"),
            Self::Human => write!(f, "Human"),
            Self::Tiefling => write!(f, "Tiefling"),
            Self::Warforged => write!(f, "Warforged"),
            Self::WoodElf => write!(f, "Wood Elf"),
            Self::Aasimar => write!(f, "Aasimar"),
            Self::Shifter => write!(f, "Shifter"),
            Self::Tabaxi => write!(f, "Tabaxi"),
            Self::Bladeforged => write!(f, "Bladeforged"),
            Self::DeepGnome => write!(f, "Deep Gnome"),
            Self::Morninglord => write!(f, "Morninglord"),
            Self::PurpleDragonKnight => write!(f, "Purple Dragon Knight"),
            Self::Razorclaw => write!(f, "Razorclaw Shifter"),
            Self::Scoundrel => write!(f, "Tiefling Scoundrel"),
            Self::Scourge => write!(f, "Aasimar Scourge"),
            Self::Shadarkai => write!(f, "Shadar-kai"),
            Self::Trailblazer => write!(f, "Tabaxi Trailblazer"),
        }
    }
}

impl ToAttribute for Race {
    fn to_attribute(self) -> Attribute {
        Flag::Race(self).to_attribute()
    }
}
