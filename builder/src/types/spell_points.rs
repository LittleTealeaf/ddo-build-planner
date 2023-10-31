use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, DefaultBonuses},
    bonus::{Bonus, BonusSource, BonusType, Value},
};

use super::PlayerClass;

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default)]
/// Different types of bonsues to spell points
pub enum SpellPoints {
    /// Bonuses that can be scaled based on the number of Favored Soul or Sorcerer levels you have
    Scaled,
    /// Regular bonuses to spell points
    #[default]
    Base,
    /// Additive modifier to total spell points
    Modifier,
    /// Total spell points
    Total,
}

impl Display for SpellPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "Spell Points"),
            Self::Modifier => write!(f, "Spell Point Modifier"),
            Self::Total => write!(f, "Total Spell Points"),
            Self::Scaled => write!(f, "Scaled Spell Points"),
        }
    }
}

impl DefaultBonuses for SpellPoints {
    fn get_default_bonuses() -> Self::Iterator {
        [
            Bonus::new(
                Attribute::SpellPoints(Self::Base),
                BonusType::Stacking,
                Value::Product(vec![
                    Attribute::SpellPoints(Self::Scaled).into(),
                    Value::Sum(vec![
                        Attribute::ClassLevel(PlayerClass::FavoredSoul).into(),
                        Attribute::ClassLevel(PlayerClass::Sorcerer).into(),
                        20f32.into(),
                    ]),
                    (1f32 / 2f32).into(),
                ]),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                Attribute::SpellPoints(Self::Total),
                BonusType::Stacking,
                Value::Product(vec![
                    Attribute::SpellPoints(Self::Base).into(),
                    Value::Sum(vec![
                        1f32.into(),
                        Attribute::SpellPoints(Self::Modifier).into(),
                    ]),
                ]),
                BonusSource::Base,
                None,
            ),
        ]
    }

    type Iterator = [Bonus; 2];
}
