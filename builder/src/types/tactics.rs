//! Tactics Bonuses
use core::fmt;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

/// Tactic Attack DCs
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tactics {
    /// Quivering Palm
    #[serde(rename = "qp", alias = "QuiveringPalm")]
    QuiveringPalm,
    /// Trip and Improved Trip bonuses
    #[serde(rename = "tr", alias = "Trip")]
    Trip,
    /// Sunder and Improved Sunder bonuses
    #[serde(rename = "su", alias = "Sunder")]
    Sunder,
    /// Assassinate DCs
    #[serde(rename = "as", alias = "Assassinate")]
    Assassinate,
    /// Stunning Fist / Stunning Blow
    #[serde(rename = "st", alias = "Stun")]
    Stun,
    /// Slicing Blow
    #[serde(rename = "sb", alias = "SlicingBlow")]
    SlicingBlow,
    /// General Tactics DC
    #[serde(rename = "al", alias = "All")]
    Tactics,
}

impl Tactics {
    /// All tactics DCs
    pub const ALL: [Self; 6] = [
        Self::Trip,
        Self::Sunder,
        Self::Assassinate,
        Self::Stun,
        Self::SlicingBlow,
        Self::QuiveringPalm,
    ];
}

impl StaticValues for Tactics {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}

impl fmt::Display for Tactics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QuiveringPalm => write!(f, "Quivering Palm"),
            Self::Trip => write!(f, "Trip"),
            Self::Sunder => write!(f, "Sunder"),
            Self::Assassinate => write!(f, "Assassinate"),
            Self::Stun => write!(f, "Stunning"),
            Self::Tactics => write!(f, "Tactics"),
            Self::SlicingBlow => write!(f, "Slicing Blow"),
        }
    }
}

impl ToAttribute for Tactics {
    fn to_attribute(self) -> Attribute {
        Attribute::Tactics(self)
    }
}

impl CloneBonus for Tactics {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::Tactics).then(|| {
            [
                Self::Trip,
                Self::Sunder,
                Self::Stun,
                Self::QuiveringPalm,
                Self::SlicingBlow,
            ]
            .map(|t| bonus.clone_with_attribute(t))
            .to_vec()
        })
    }
}
