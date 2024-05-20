//! Healing Amplification Types

use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

/// Types of Healing Amplification
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HealingAmplification {
    /// Positive Healing Amplification
    #[serde(rename = "p", alias = "Pos", alias = "Positive")]
    Positive,
    /// Negative Healing Amplification
    #[serde(rename = "n", alias = "Neg", alias = "Negative")]
    Negative,
    /// Repair Healing Amplification
    #[serde(rename = "r", alias = "Rep", alias = "Repair")]
    Repair,
    /// All Healing Amplification
    #[serde(rename = "a", alias = "All")]
    All,
}

impl HealingAmplification {
    /// All three channels of healing amplification
    pub const ALL: [Self; 3] = [Self::Positive, Self::Negative, Self::Repair];
}

impl Display for HealingAmplification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positive => write!(f, "Positive Healing Amplification"),
            Self::Negative => write!(f, "Negative Healing Amplification"),
            Self::Repair => write!(f, "Repair Amplification"),
            Self::All => write!(f, "Healing Amplification"),
        }
    }
}

impl ToAttribute for HealingAmplification {
    fn to_attribute(self) -> Attribute {
        Attribute::HealingAmplification(self)
    }
}

impl CloneBonus for HealingAmplification {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::ALL
                .map(|amp| bonus.clone_with_attribute(amp))
                .to_vec()
        })
    }
}

impl StaticOptions for HealingAmplification {
    fn get_static() -> impl Iterator<Item = Self> {
        [Self::Positive, Self::Negative, Self::Repair, Self::All].into_iter()
    }
}
