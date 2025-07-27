//! Contains player race
public_modules!(bonuses);

use core::fmt::{self, Display};
use serde::{Deserialize, Serialize};
use utils::{enums::StaticValues, public_modules};

use crate::attribute::{Attribute, ToAttribute};

use super::flag::{Flag, ToFlag};

/// The different race options that the character can be.
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Race {
    /// Dragonborn Race
    #[serde(rename = "db", alias = "Dragonborn")]
    Dragonborn,
    /// Drow Race
    #[serde(rename = "dr", alias = "Drow")]
    Drow,
    /// Dwarf Race
    #[serde(rename = "dw", alias = "Dwarf")]
    Dwarf,
    /// Elf Race
    #[serde(rename = "el", alias = "Elf")]
    Elf,
    /// Gnome Race
    #[serde(rename = "gn", alias = "Gnome")]
    Gnome,
    /// Halfling Race
    #[serde(rename = "hl", alias = "Halfling")]
    Halfling,
    /// Half-Elf Race
    #[serde(rename = "he", alias = "HalfElf")]
    HalfElf,
    /// Half-Orc Race
    #[serde(rename = "ho", alias = "HalfOrc")]
    HalfOrc,
    /// Human Race
    #[serde(rename = "h", alias = "Human")]
    Human,
    /// Tiefling Race
    #[serde(rename = "ti", alias = "Tiefling")]
    Tiefling,
    /// Warforged Race
    #[serde(rename = "wf", alias = "Warforged")]
    Warforged,
    /// Wood-Elf Race
    #[serde(rename = "we", alias = "WoodElf")]
    WoodElf,
    /// Aasimar Race
    #[serde(rename = "a", alias = "Aasimar")]
    Aasimar,
    /// Shifter Race
    #[serde(rename = "sh", alias = "Shifter")]
    Shifter,
    /// Tabaxi Race
    #[serde(rename = "ta", alias = "Tabaxi")]
    Tabaxi,
    /// Bladeforged Race
    #[serde(rename = "bf", alias = "Bladeforged")]
    Bladeforged,
    /// Deep-Gnome Race
    #[serde(rename = "dg", alias = "DeepGnome")]
    DeepGnome,
    /// Morninglord Race
    #[serde(rename = "ml", alias = "Morninglord")]
    Morninglord,
    /// Purple Dragon Knight Race
    #[serde(rename = "PDK", alias = "PurpleDragonKnight")]
    PurpleDragonKnight,
    /// Razorclaw Race
    #[serde(rename = "rc", alias = "Razorclaw")]
    Razorclaw,
    /// Scoundrel Race
    #[serde(rename = "sc", alias = "Scoundrel")]
    Scoundrel,
    /// Scourge Race
    #[serde(rename = "so", alias = "Scourge")]
    Scourge,
    /// Shadarkai Race
    #[serde(rename = "sk", alias = "Shadarkai")]
    Shadarkai,
    /// Trailblazer Race
    #[serde(rename = "tr", alias = "Trailblaze")]
    Trailblazer,
    /// Eladrin
    #[serde(rename = "ea", alias = "Eladrin")]
    Eladrin,
    /// Eladrin Chaosmancer
    #[serde(rename = "cm", alias = "Chaosmancer")]
    Chaosmancer,
    /// Dhampir
    #[serde(rename = "dh", alias = "Dhampir")]
    Dhampir,
    /// Dark Dhampir Bargainer
    #[serde(rename = "ba", alias = "Bargainer")]
    Bargainer,
}

impl Race {
    /// All Races
    pub const ALL: [Self; 28] = [
        Self::Dragonborn,
        Self::Drow,
        Self::Dwarf,
        Self::Elf,
        Self::Gnome,
        Self::Halfling,
        Self::HalfElf,
        Self::HalfOrc,
        Self::Human,
        Self::Tiefling,
        Self::Warforged,
        Self::WoodElf,
        Self::Aasimar,
        Self::Shifter,
        Self::Tabaxi,
        Self::Bladeforged,
        Self::DeepGnome,
        Self::Morninglord,
        Self::PurpleDragonKnight,
        Self::Razorclaw,
        Self::Scoundrel,
        Self::Scourge,
        Self::Shadarkai,
        Self::Trailblazer,
        Self::Eladrin,
        Self::Chaosmancer,
        Self::Dhampir,
        Self::Bladeforged,
    ];
}

impl Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
            Self::Eladrin => write!(f, "Eladrin"),
            Self::Chaosmancer => write!(f, "Chaosmancer"),
            Self::Dhampir => write!(f, "Dhampir"),
            Self::Bargainer => write!(f, "Dark Dhampir Bargainer"),
        }
    }
}

impl ToAttribute for Race {
    fn to_attribute(self) -> Attribute {
        self.to_flag().to_attribute()
    }
}

impl ToFlag for Race {
    fn to_flag(self) -> Flag {
        Flag::Race(self)
    }
}

impl StaticValues for Race {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
