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
    pub const fn is_iconic(self) -> bool {
        matches!(
            self,
            Self::ShadarKai
                | Self::TabaxiTrailblazer
                | Self::RazorclawShifter
                | Self::DeepGnome
                | Self::Chaosmancer
                | Self::AasimarScourge
                | Self::DarkBargainer
                | Self::Morninglord
                | Self::PurpleDragonKnight
                | Self::Scoundrel
                | Self::DuergarMindcleaver
        )
    }

    #[must_use]
    pub const fn past_life(self) -> PastLife {
        PastLife::Racial(self)
    }

    #[must_use]
    pub const fn past_life_skill(self) -> Option<Skill> {
        match self {
            Self::Human => Some(Skill::Haggle),
            Self::Elf | Self::Shifter => Some(Skill::Spot),
            Self::Halfling => Some(Skill::MoveSilently),
            Self::Dwarf | Self::Duergar => Some(Skill::Balance),
            Self::Dragonborn | Self::Tiefling => Some(Skill::Spellcraft),
            Self::DrowElf => Some(Skill::Search),
            Self::Gnome => Some(Skill::UseMagicalDevice),
            Self::HalfElf => Some(Skill::Diplomacy),
            Self::HalfOrc => Some(Skill::Intimidate),
            Self::Aasimar => Some(Skill::Heal),
            Self::Dhampir => Some(Skill::Hide),
            Self::Eladrin => Some(Skill::Listen),
            Self::Tabaxi => Some(Skill::Tumble),
            Self::Warforged => Some(Skill::Repair),
            _ => None,
        }
    }

    #[must_use]
    pub const fn past_life_ability(self) -> Option<Ability> {
        match self {
            Self::Human => todo!(),
            Self::Elf => todo!(),
            Self::WoodElf => todo!(),
            Self::Halfling => todo!(),
            Self::Dwarf => todo!(),
            Self::Dragonborn => todo!(),
            Self::DrowElf => todo!(),
            Self::Gnome => todo!(),
            Self::HalfElf => todo!(),
            Self::HalfOrc => todo!(),
            Self::Tiefling => todo!(),
            Self::Aasimar => todo!(),
            Self::Dhampir => todo!(),
            Self::Duergar => todo!(),
            Self::Eladrin => todo!(),
            Self::Shifter => todo!(),
            Self::Tabaxi => todo!(),
            Self::Warforged => todo!(),
            _ => None,
        }
    }
}
