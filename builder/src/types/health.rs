//! Player Health
use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::attribute::{Attribute, ToAttribute};

#[derive(
    Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
/// Indicates bonuses to hit points, or health
pub enum Health {
    /// Base hit points, prior to any bonuses from combat style feats
    #[serde(rename = "B", alias = "Base")]
    Base,
    /// Bonuses from combat style feats
    #[serde(rename = "bm", alias = "BaseModifier")]
    BaseModifier,
    /// Bonus hit points, this is the standard one
    #[serde(rename = "b", alias = "Bonus")]
    #[default]
    Bonus,
    /// Final modifier over all hit points
    #[serde(rename = "m", alias = "Modifier")]
    Modifier,
    /// Total hit points
    #[serde(rename = "t", alias = "Total")]
    Total,
}

impl Health {
    /// All possible values
    pub const VALUES: [Self; 5] = [
        Self::Base,
        Self::BaseModifier,
        Self::Bonus,
        Self::Modifier,
        Self::Total,
    ];
}

impl Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn to_attribute(self) -> Attribute {
        Attribute::Health(self)
    }
}

impl StaticValues for Health {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
