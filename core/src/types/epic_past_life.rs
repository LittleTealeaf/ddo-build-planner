use crate::{traits::IterValues, types::past_life::PastLife};

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
    derive_more::From,
    derive_more::Display,
    strum::VariantArray,
)]
pub enum EpicPastLife {
    AncientKnowledge,
    ArcaneAlacrity,
    EclcipsePower,
    EnchantWeapon,
    EnergyCriticals,
    AncientBlessings,
    BlockEnergy,
    Brace,
    PowerOverLifeAndDeath,
    AncientTactics,
    Doublestrike,
    Fortification,
    SkillMastery,
    TrapDamage,
    AncientPower,
    ColorsOfTheQueen,
    Doubleshot,
    FastHealing,
}

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
    derive_more::From,
    derive_more::Display,
    strum::VariantArray,
)]
pub enum EpicSphere {
    Arcane,
    Divine,
    Primal,
    Martial,
}

impl EpicPastLife {
    #[must_use]
    pub const fn past_life(self) -> PastLife {
        PastLife::Epic(self)
    }

    #[must_use]
    pub const fn sphere(self) -> EpicSphere {
        match self {
            Self::AncientKnowledge
            | Self::ArcaneAlacrity
            | Self::EclcipsePower
            | Self::EnchantWeapon
            | Self::EnergyCriticals => EpicSphere::Arcane,
            Self::AncientBlessings
            | Self::BlockEnergy
            | Self::Brace
            | Self::PowerOverLifeAndDeath => EpicSphere::Divine,
            Self::AncientTactics
            | Self::Doublestrike
            | Self::Fortification
            | Self::SkillMastery
            | Self::TrapDamage => EpicSphere::Martial,
            Self::AncientPower | Self::ColorsOfTheQueen | Self::Doubleshot | Self::FastHealing => {
                EpicSphere::Primal
            }
        }
    }
}

impl EpicSphere {
    pub fn past_lives(self) -> impl Iterator<Item = EpicPastLife> {
        EpicPastLife::values().filter(move |pl| pl.sphere() == self)
    }
}
