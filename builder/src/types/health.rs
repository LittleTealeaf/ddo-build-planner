use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, DefaultBonuses},
    bonus::{Bonus, BonusSource, BonusType, Value},
};

#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default)]
/// Indicates bonuses to hit points, or health
pub enum Health {
    /// Base hit points, prior to any bonuses from combat style feats
    Base,
    /// Bonuses from combat style feats
    BaseModifier,
    /// Bonus hit points, this is the standard one
    #[default]
    Bonus,
    /// Final modifier over all hit points
    Modifier,
    /// Total hit points
    Total,
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "Base Health"),
            Self::BaseModifier => write!(f, "Base Health Modifier"),
            Self::Bonus => write!(f, "Bonus Health"),
            Self::Modifier => write!(f, "Health Modifier"),
            Self::Total => write!(f, "Total"),
        }
    }
}

impl DefaultBonuses for Health {
    fn get_default_bonuses() -> Self::Iterator {
        [
            Bonus::new(
                Attribute::Health(Health::Bonus),
                BonusType::Stacking,
                Value::Product(vec![
                    Attribute::Health(Health::Base).into(),
                    Value::Sum(vec![
                        Attribute::Health(Health::BaseModifier).into(),
                        1f32.into(),
                    ]),
                ]),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                Attribute::Health(Health::Total),
                BonusType::Stacking,
                Value::Product(vec![
                    Attribute::Health(Health::Bonus).into(),
                    Value::Sum(vec![
                        Attribute::Health(Health::Modifier).into(),
                        1f32.into(),
                    ]),
                ]),
                BonusSource::Base,
                None,
            ),
        ]
    }

    type Iterator = [Bonus; 2];
}
