//! Spell Points
use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

#[derive(
    Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
/// Different types of bonuses to spell points
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "Spell Points"),
            Self::Modifier => write!(f, "Spell Point Modifier"),
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

impl StaticOptions for SpellPoints {
    fn get_static() -> impl Iterator<Item = Self> {
        [Self::Scaled, Self::Base, Self::Modifier, Self::Total].into_iter()
    }
}
