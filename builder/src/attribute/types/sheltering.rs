use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
};

/// Sheltering attributes grant a % reduction to damage from that type.
///
/// Magical Sheltering can be capped at a certain amount based on equipment and enhancements, which is tracked with [`Sheltering::MagicalCap`]
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Sheltering {
    /// Both [`Physical`] and [`Magical`] Sheltering
    ///
    /// [`Physical`]: Sheltering::Physical
    /// [`Magical`]: Sheltering::Magical
    Both,
    /// Physical Sheltering
    Physical,
    /// Magical Sheltering
    Magical,
    /// Magical Sheltering Cap
    MagicalCap,
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
            Self::Physical => write!(f, "Physical Sheltering"),
            Self::Magical => write!(f, "Magical Sheltering"),
            Self::MagicalCap => write!(f, "Magical Sheltering Cap"),
            Self::Both => write!(f, "Sheltering"),
        }
    }
}

impl TrackAttribute for Sheltering {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::Both)
    }
}
