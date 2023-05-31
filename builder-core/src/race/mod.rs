mod display;

use enum_map::Enum;

pub use display::*;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
pub enum Race {
    Dragonborn,
    Drow,
    Dwarf,
    Elf,
    Gnome,
    Halfling,
    HalfElf,
    HalfOrc,
    Human,
    Tiefling,
    Warforged,
    WoodElf,
    Aasimar,
    Shifter,
    Tabaxi,
    Bladeforged,
    DeepGnome,
    Morninglord,
    PurpleDragonKnight,
    Razorclaw,
    Scoundrel,
    Scourge,
    Shadarkai,
    Trailblazer
}
