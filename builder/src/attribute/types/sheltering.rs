use std::fmt::Display;

use enum_map::Enum;
use serde::{Serialize, Deserialize};

use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, CloneBonus},
};

/// Sheltering attributes grant a % reduction to damage from that type.
///
/// Magical Sheltering can be capped at a certain amount based on equipment and enhancements, which is tracked with [`Sheltering::MagicalCap`]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Sheltering {
    /// Physical Sheltering
    Physical,
    /// Magical Sheltering
    Magical,
    /// Magical Sheltering Cap
    MagicalCap,
    /// Both [`Physical`] and [`Magical`] Sheltering
    ///
    /// [`Physical`]: Sheltering::Physical
    /// [`Magical`]: Sheltering::Magical
    Both,
}

impl CloneBonus for Sheltering {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::Both).then(|| {
            [Self::Physical, Self::Magical]
                .map(|sheltering| {
                    Bonus::new(
                        sheltering.into(),
                        bonus.get_type(),
                        bonus.get_value(),
                        bonus.get_source(),
                        bonus.get_condition(),
                    )
                })
                .to_vec()
        })
    }
}

impl Display for Sheltering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sheltering::Physical => write!(f, "Physical Sheltering"),
            Sheltering::Magical => write!(f, "Magical Sheltering"),
            Sheltering::MagicalCap => write!(f, "Magical Sheltering Cap"),
            Sheltering::Both => write!(f, "Sheltering"),
        }
    }
}

impl TrackAttribute for Sheltering {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::Both)
    }
}

impl From<Sheltering> for Attribute {
    fn from(value: Sheltering) -> Self {
        Self::Sheltering(value)
    }
}
