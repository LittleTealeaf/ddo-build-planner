//! Spell Points
use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::attribute::{Attribute, ToAttribute};

#[derive(
    Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
/// Different types of bonuses to spell points
pub enum SpellPoints {
    /// Bonuses that can be scaled based on the number of Favored Soul or Sorcerer levels you have
    #[serde(rename = "s", alias = "Scaled")]
    Scaled,
    /// Regular bonuses to spell points
    #[default]
    #[serde(rename = "b", alias = "Base")]
    Base,
    /// Additive modifier to total spell points
    #[serde(rename = "m", alias="Modifier", alias = "Scalar")]
    Scalar,
    /// Total spell points
    #[serde(rename = "t", alias = "Total")]
    Total,
}

impl Display for SpellPoints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "Spell Points"),
            Self::Scalar => write!(f, "Spell Point Scalar"),
            Self::Total => write!(f, "Total Spell Points"),
            Self::Scaled => write!(f, "Scaled Spell Points"),
        }
    }
}

impl ToAttribute for SpellPoints {
    fn to_attribute(self) -> Attribute {
        Attribute::SpellPoints(self)
    }
}

impl StaticValues for SpellPoints {
    fn values() -> impl Iterator<Item = Self> {
        [Self::Scaled, Self::Base, Self::Scalar, Self::Total].into_iter()
    }
}
