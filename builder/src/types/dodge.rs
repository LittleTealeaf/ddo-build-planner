//! Dodge Types

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::attribute::{Attribute, ToAttribute};

/// Dodge, Dodge Cap, and Total Dodge
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Dodge {
    /// Dodge Bonuses
    #[serde(rename = "b", alias = "Dodge", alias = "Bonus")]
    Bonus,
    /// Bonuses to Dodge Cap
    #[serde(rename = "c", alias = "Cap")]
    Cap,
    /// Uncapped Bonuses
    #[serde(rename = "u", alias = "Uncapped")]
    Uncapped,
    /// Effective Final Dodge
    #[serde(rename = "t", alias = "Total")]
    Total,
}

impl Dodge {
    /// All values
    pub const ALL: [Self; 4] = [Self::Bonus, Self::Cap, Self::Total, Self::Uncapped];
}

impl Display for Dodge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bonus => write!(f, "Dodge Bonus"),
            Self::Cap => write!(f, "Dodge Cap"),
            Self::Total => write!(f, "Total Dodge"),
            Self::Uncapped => write!(f, "Uncapped Dodge Bonus"),
        }
    }
}

impl ToAttribute for Dodge {
    fn to_attribute(self) -> Attribute {
        Attribute::Dodge(self)
    }
}

impl StaticValues for Dodge {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
