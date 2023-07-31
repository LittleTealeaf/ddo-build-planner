use std::fmt::Display;

use super::Race;

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
