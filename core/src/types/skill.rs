use itertools::chain;

use crate::{
    attribute::Attribute,
    bonus::{traits::ToValue, Bonus, BonusType},
    traits::IterValues,
    types::ability::Ability,
    val,
};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum Skill {
    Balance,
    Bluff,
    Concentration,
    Diplomacy,
    #[display("Disable Device")]
    DisableDevice,
    Haggle,
    Heal,
    Hide,
    Intimidate,
    Jump,
    Listen,
    #[display("Move Silently")]
    MoveSilently,
    #[display("Open Lock")]
    OpenLock,
    Perform,
    Repair,
    Search,
    Spellcraft,
    Spot,
    Swim,
    Tumble,
    #[display("Use Magical Device")]
    UseMagicalDevice,
}

impl Skill {
    pub const VALUES: [Self; 21] = [
        Self::Balance,
        Self::Bluff,
        Self::Concentration,
        Self::Diplomacy,
        Self::DisableDevice,
        Self::Haggle,
        Self::Heal,
        Self::Hide,
        Self::Intimidate,
        Self::Jump,
        Self::Listen,
        Self::MoveSilently,
        Self::OpenLock,
        Self::Perform,
        Self::Repair,
        Self::Search,
        Self::Spellcraft,
        Self::Spot,
        Self::Swim,
        Self::Tumble,
        Self::UseMagicalDevice,
    ];

    #[must_use]
    pub const fn ability(self) -> Ability {
        match self {
            Self::Jump | Self::Swim => Ability::Strength,
            Self::Balance | Self::Hide | Self::MoveSilently | Self::OpenLock | Self::Tumble => {
                Ability::Dexterity
            }
            Self::Concentration => Ability::Constitution,
            Self::DisableDevice | Self::Repair | Self::Search | Self::Spellcraft => {
                Ability::Intelligence
            }
            Self::Heal | Self::Listen | Self::Spot => Ability::Wisdom,
            Self::Bluff
            | Self::Diplomacy
            | Self::Haggle
            | Self::Intimidate
            | Self::Perform
            | Self::UseMagicalDevice => Ability::Charisma,
        }
    }
}

impl IterValues for Skill {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}

impl Skill {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        chain!(
            Self::values().map(|skill| {
                Bonus::new(
                    skill,
                    skill.ability().modifier(),
                    BonusType::Ability,
                    skill.ability().modifier(),
                )
            }),
            [
                (Self::Balance, val!(-1)),
                (Self::Hide, val!(-1)),
                (Self::Jump, val!(-1)),
                (Self::MoveSilently, val!(-1)),
                (Self::Swim, val!(-2)),
                (Self::Tumble, val!(-1)),
            ]
            .into_iter()
            .map(|(skill, scale)| {
                Bonus::new(
                    skill,
                    scale * Attribute::ArmorCheckPenalty.to_value(),
                    BonusType::Stacking,
                    Attribute::ArmorCheckPenalty,
                )
            }),
        )
    }
}
