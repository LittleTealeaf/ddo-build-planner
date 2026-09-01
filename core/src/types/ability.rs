use crate::{
    attribute::Attribute,
    bonus::{traits::ToValue, Bonus, BonusType},
    traits::IterValues,
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
    strum::VariantArray,
)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability {
    #[must_use]
    pub const fn score(self) -> Attribute {
        Attribute::AbilityScore(self)
    }

    #[must_use]
    pub const fn modifier(self) -> Attribute {
        Attribute::AbilityModifier(self)
    }
}

impl Ability {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        Self::values().flat_map(|ability| {
            [
                Bonus::new(
                    ability.modifier(),
                    (ability.score().to_value().max(val!(0)) / val!(2)).floor() - val!(5),
                    BonusType::Ability,
                    ability.score(),
                ),
                Bonus::new(
                    ability.score(),
                    val!(8),
                    BonusType::Stacking,
                    ability.score(),
                ),
            ]
        })
    }
}
