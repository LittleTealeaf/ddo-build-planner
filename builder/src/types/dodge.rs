//! Dodge in the game

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

/// Provides bonuses to dodge, which is a chance that an attack's damage will be completely ignored
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Dodge {
    /// Bonus Dodce
    Bonus,
    /// Bonuses to Dodge Cap
    CapBonus,
    /// Calculated Bonus Cap (Do not touch)
    Cap,
    /// Total Dodge
    Total,
}

impl Display for Dodge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bonus => write!(f, "Dodge Bonus"),
            Self::CapBonus => write!(f, "Bonus Dodge Cap"),
            Self::Total => write!(f, "Dodge"),
            Self::Cap => write!(f, "Dodge Cap"),
        }
    }
}

impl ToAttribute for Dodge {
    fn to_attribute(self) -> Attribute {
        Attribute::Dodge(self)
    }
}

impl StaticOptions for Dodge {
    fn get_static() -> impl Iterator<Item = Self> {
        [Self::Bonus, Self::CapBonus, Self::Cap, Self::Total].into_iter()
    }
}
