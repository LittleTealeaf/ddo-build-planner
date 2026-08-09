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
    pub const VALUES: [Self; 6] = [
        Self::Strength,
        Self::Dexterity,
        Self::Constitution,
        Self::Intelligence,
        Self::Wisdom,
        Self::Charisma,
    ];

    #[must_use]
    pub const fn score(self) -> Attribute {
        Attribute::AbilityScore(self)
    }

    #[must_use]
    pub const fn modifier(self) -> Attribute {
        Attribute::AbilityModifier(self)
    }
}

impl IterValues for Ability {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
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
