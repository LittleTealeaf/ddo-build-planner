use crate::{
    bonus::{Bonus, BonusType},
    traits::IterValues,
    types::ability::Ability,
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
pub enum SavingThrow {
    Fortitude,
    Reflex,
    Will,
    Poison,
    Disease,
    Traps,
    Spell,
    Magic,
    Enchantment,
    Illusion,
    Fear,
    Curse,
}

impl SavingThrow {
    const VALUES: [Self; 11] = [
        Self::Fortitude,
        Self::Reflex,
        Self::Will,
        Self::Poison,
        Self::Traps,
        Self::Spell,
        Self::Magic,
        Self::Enchantment,
        Self::Illusion,
        Self::Fear,
        Self::Curse,
    ];

    #[must_use]
    pub const fn ability(self) -> Option<Ability> {
        match self {
            Self::Reflex => Some(Ability::Dexterity),
            Self::Will => Some(Ability::Wisdom),
            Self::Fortitude => Some(Ability::Constitution),
            _ => None,
        }
    }
}

impl IterValues for SavingThrow {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}

impl SavingThrow {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        Self::values().filter_map(|save| {
            Some(Bonus::new(
                save,
                save.ability()?.modifier(),
                BonusType::Ability,
                save.ability()?.modifier(),
            ))
        })
    }
}
