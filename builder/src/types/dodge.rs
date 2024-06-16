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
    /// Dodge Cap
    #[serde(rename = "c", alias = "Cap")]
    Cap,
    /// Effective Final Dodge
    #[serde(rename = "t", alias = "Total")]
    Total,
}

impl Dodge {
    /// All values
    pub const ALL: [Self; 3] = [Self::Bonus, Self::Cap, Self::Total];
}

impl Display for Dodge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bonus => write!(f, "Dodge Bonus"),
            Self::Cap => write!(f, "Dodge Cap"),
            Self::Total => write!(f, "Total Dodge"),
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
