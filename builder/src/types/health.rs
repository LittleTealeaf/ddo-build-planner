//! Player Health
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default)]
/// Indicates bonuses to hit points, or health
pub enum Health {
    /// Base hit points, prior to any bonuses from combat style feats
    Base,
    /// Bonuses from combat style feats
    BaseModifier,
    /// Bonus hit points, this is the standard one
    #[default]
    Bonus,
    /// Final modifier over all hit points
    Modifier,
    /// Total hit points
    Total,
}

impl Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "Base Health"),
            Self::BaseModifier => write!(f, "Base Health Modifier"),
            Self::Bonus => write!(f, "Bonus Health"),
            Self::Modifier => write!(f, "Health Modifier"),
            Self::Total => write!(f, "Total"),
        }
    }
}
