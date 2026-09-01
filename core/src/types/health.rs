use itertools::chain;

use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusSource, BonusType, BonusValue,
    },
    traits::IterValues,
    types::{ability::Ability, level::PlayerLevel, player_class::PlayerClass},
    val,
};

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
    derive_more::From,
    strum::VariantArray,
)]
pub enum PlayerHealth {
    Base,
    BaseMultiplier,
    Bonus,
    Multiplier,
    Total,
}

impl PlayerHealth {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        chain!(
            [
                Bonus::new(
                    Self::Bonus,
                    Self::Base.attribute().to_value()
                        * Self::BaseMultiplier.attribute().to_value().as_multiplier(),
                    BonusType::Stacking,
                    BonusSource::Custom("Base Health".to_owned()),
                ),
                Bonus::new(
                    Self::Total,
                    Self::Bonus.attribute().to_value()
                        * Self::Multiplier.attribute().to_value().as_multiplier(),
                    BonusType::Stacking,
                    BonusSource::Custom("Base Health".to_owned()),
                ),
                Bonus::new(
                    Self::Base,
                    PlayerLevel::Total.attribute().to_value()
                        * Ability::Constitution.modifier().to_value(),
                    BonusType::Stacking,
                    BonusSource::Attribute(Ability::Constitution.modifier())
                ),
                Bonus::new(
                    Self::Base,
                    PlayerLevel::Epic.attribute().to_value() * val!(10),
                    BonusType::Stacking,
                    BonusSource::Attribute(PlayerLevel::Epic.attribute())
                ),
                Bonus::new(
                    Self::Base,
                    PlayerLevel::Legendary.attribute().to_value() * val!(10),
                    BonusType::Stacking,
                    BonusSource::Attribute(PlayerLevel::Legendary.attribute())
                )
            ],
            PlayerClass::values().map(|cls| Bonus::new(
                Self::Base,
                cls.level().to_value() * BonusValue::from(cls.hit_points_per_level()),
                BonusType::Stacking,
                BonusSource::Attribute(cls.level())
            )
            .with_show_condition(
                PlayerLevel::from(cls)
                    .attribute()
                    .to_value()
                    .greater_than(val!(0))
            ))
        )
    }
}
