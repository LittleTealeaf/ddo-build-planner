//! Player Health
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

#[derive(
    Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
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
            Self::Base => write!(f, "Base Hit Points"),
            Self::BaseModifier => write!(f, "Base Hit Points Modifier"),
            Self::Bonus => write!(f, "Bonus Hit Points"),
            Self::Modifier => write!(f, "Hit Point Modifier"),
            Self::Total => write!(f, "Total Hit Points"),
        }
    }
}

impl ToAttribute for Health {
    fn to_attribute(self) -> crate::attribute::Attribute {
        Attribute::Health(self)
    }
}

impl StaticOptions for Health {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Self::Base,
            Self::BaseModifier,
            Self::Bonus,
            Self::Modifier,
            Self::Total,
        ]
        .into_iter()
    }
}
