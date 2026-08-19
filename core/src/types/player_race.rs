use strum::VariantArray;

use crate::types::{ability::Ability, past_life::PastLife, skill::Skill};

#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    VariantArray,
)]
pub enum PlayerRace {
    Human,
    Elf,
    #[display("Wood Elf")]
    WoodElf,
    Halfling,
    Dwarf,
    Dragonborn,
    #[display("Drow Elf")]
    DrowElf,
    Gnome,
    #[display("Half-Elf")]
    HalfElf,
    #[display("Half-Orc")]
    HalfOrc,
    Tiefling,
    Aasimar,
    Dhampir,
    Duergar,
    Eladrin,
    Shifter,
    Tabaxi,
    Warforged,
    #[display("Aasimar Scourge")]
    AasimarScourge,
    Bladeforged,
    Chaosmancer,
    #[display("Dark Bargainer")]
    DarkBargainer,
    #[display("Deep Gnome")]
    DeepGnome,
    #[display("Duergar Mindcleaver")]
    DuergarMindcleaver,
    #[display("Morninglord")]
    Morninglord,
    #[display("Purple Dragon Knight")]
    PurpleDragonKnight,
    #[display("Razorclaw Shifter")]
    RazorclawShifter,
    Scoundrel,
    ShadarKai,
    #[display("Tabaxi Trailblazer")]
    TabaxiTrailblazer,
}

impl PlayerRace {
    #[must_use]
    pub const fn get_parent_race(self) -> Option<Self> {
        match self {
            Self::WoodElf => Some(Self::Elf),
            _ => None,
        }
    }

    #[must_use]
    pub const fn past_life(self) -> PastLife {
        PastLife::Racial(self)
    }

    pub const fn past_life_skill(self) -> Skill {
        todo!()
    }

    pub const fn past_life_ability(self) -> Ability {
        todo!()
    }
}
