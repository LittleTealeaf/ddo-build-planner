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
    strum::VariantArray,
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
