//! Healing Amplifcation Types

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::CloneBonus,
};

/// Types of Healing Amplification
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HealingAmplification {
    /// Positive Healing Amplification
    Positive,
    /// Negative Healing Amplification
    Negative,
    /// Repair Healing Amplification
    Repair,
    /// All Healing Amplification
    All,
}

impl HealingAmplification {
    /// All three channels of healing amplification
    pub const ALL: [Self; 3] = [Self::Positive, Self::Negative, Self::Repair];
}

impl Display for HealingAmplification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive => write!(f, "Positive Healing Amplification"),
            Self::Negative => write!(f, "Negative Healing Amplification"),
            Self::Repair => write!(f, "Repair Amplification"),
            Self::All => write!(f, "Healing Amplification"),
        }
    }
}

impl ToAttribute for HealingAmplification {
    fn to_attribute(self) -> crate::attribute::Attribute {
        Attribute::HealingAmplification(self)
    }
}

impl CloneBonus for HealingAmplification {
    fn clone_bonus(&self, bonus: &crate::bonus::Bonus) -> Option<Vec<crate::bonus::Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::ALL
                .map(|amp| bonus.clone_into_attribute(amp))
                .to_vec()
        })
    }
}
