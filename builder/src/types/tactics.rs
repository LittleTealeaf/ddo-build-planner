//! Tactics Bonuses
use core::fmt;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

/// Tactic Attack DCs
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tactics {
    /// Trip and Improved Trip bonuses
    #[serde(rename="tr", alias = "Trip")]
    Trip,
    /// Sunder and Improved Sunder bonuses
    #[serde(rename="su", alias = "Sunder")]
    Sunder,
    /// Assassinate DCs
    #[serde(rename="as", alias = "Assassinate")]
    Assassinate,
    /// Stunning Fist / Stunning Blow
    #[serde(rename="st", alias = "Stun")]
    Stun,
    /// General Tactics DC
    #[serde(rename="al", alias = "All")]
    All,
}

impl Tactics {
    /// All tactics DCs
    pub const ALL: [Tactics; 4] = [
        Self::Trip,
        Self::Sunder,
        Self::Assassinate,
        Self::Stun,
    ];
}

impl StaticOptions for Tactics {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::ALL.iter().cloned()
    }
}

impl fmt::Display for Tactics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Trip => write!(f, "Trip"),
            Self::Sunder => write!(f, "Sunder"),
            Self::Assassinate => write!(f, "Assassinate"),
            Self::Stun => write!(f, "Stunning"),
            Self::All => write!(f, "All"),
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
        matches!(self, Self::All).then(|| {
            [Self::Trip, Self::Sunder, Self::Stun]
                .map(|t| bonus.clone_into_attribute(t))
                .to_vec()
        })
    }
}
