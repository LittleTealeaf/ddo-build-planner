use std::fmt::Display;

use super::Race;

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Race::Dragonborn => write!(f, "Dragonborn"),
            Race::Drow => write!(f, "Drow"),
            Race::Dwarf => write!(f, "Dwarf"),
            Race::Elf => write!(f, "Elf"),
            Race::Gnome => write!(f, "Gnome"),
            Race::Halfling => write!(f, "Halfling"),
            Race::HalfElf => write!(f, "Half Elf"),
            Race::HalfOrc => write!(f, "Half Orc"),
            Race::Human => write!(f, "Human"),
            Race::Tiefling => write!(f, "Tiefling"),
            Race::Warforged => write!(f, "Warforged"),
            Race::WoodElf => write!(f, "Wood Elf"),
            Race::Aasimar => write!(f, "Aasimar"),
            Race::Shifter => write!(f, "Shifter"),
            Race::Tabaxi => write!(f, "Tabaxi"),
            Race::Bladeforged => write!(f, "Bladeforged"),
            Race::DeepGnome => write!(f, "Deep Gnome"),
            Race::Morninglord => write!(f, "Morninglord"),
            Race::PurpleDragonKnight => write!(f, "Purple Dragon Knight"),
            Race::Razorclaw => write!(f, "Razorclaw Shifter"),
            Race::Scoundrel => write!(f, "Tiefling Scoundrel"),
            Race::Scourge => write!(f, "Aasimar Scourge"),
            Race::Shadarkai => write!(f, "Shadar-kai"),
            Race::Trailblazer => write!(f, "Tabaxi Trailblazer"),
        }
    }
}
