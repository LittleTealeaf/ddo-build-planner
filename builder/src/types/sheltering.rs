//! Describes Physical and Magical Sheltering
use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

/// Sheltering attributes grant a % reduction to damage from that type.
///
/// Magical Sheltering can be capped at a certain amount based on equipment and enhancements, which is tracked with [`Sheltering::MagicalCap`]
#[derive(
    Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
pub enum Sheltering {
    /// Both [`Physical`] and [`Magical`] Sheltering
    ///
    /// [`Physical`]: Sheltering::Physical
    /// [`Magical`]: Sheltering::Magical
    #[default]
    #[serde(rename = "b", alias = "Both")]
    Both,
    /// Physical Sheltering
    #[serde(rename = "p", alias = "Physical")]
    Physical,
    /// Magical Sheltering
    #[serde(rename = "m", alias = "Magical")]
    Magical,
    /// Magical Sheltering Cap
    #[serde(rename = "c", alias = "MagicalCap")]
    MagicalCap,
    /// Final Physical Sheltering value. DO NOT MANUALLY INCREASE. Use [`Physical`] instead.
    ///
    /// [`Physical`]: Sheltering::Physical
    PhysicalTotal,
    /// Final Magical Sheltering value. DO NOT MANUALLY INCREASE. Use [`Magical`] instead.
    ///
    ///[`Magical`]: Sheltering::Magical
    MagicalTotal,
    /// % of damage reduced by Physical Sheltering
    ///
    /// Value is from 0% - 100% (0-100)
    PhysicalReduction,
    /// % of damage reduced by Magical Sheltering.
    ///
    /// Value is from 0% - 100% (0-100)
    MagicalReduction,
}

impl Display for Sheltering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Physical => write!(f, "Physical Sheltering"),
            Self::Magical => write!(f, "Magical Sheltering"),
            Self::MagicalCap => write!(f, "Magical Sheltering Cap"),
            Self::Both => write!(f, "Sheltering"),
            Self::MagicalTotal => write!(f, "Magical Sheltering Total"),
            Self::PhysicalTotal => write!(f, "Physical Sheltering Total"),
            Self::MagicalReduction => write!(f, "Magical Reduction"),
            Self::PhysicalReduction => write!(f, "Physical Reduction"),
        }
    }
}

impl ToAttribute for Sheltering {
    fn to_attribute(self) -> Attribute {
        Attribute::Sheltering(self)
    }
}

impl CloneBonus for Sheltering {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::Both).then(|| {
            [Self::Physical, Self::Magical]
                .map(|sheltering| bonus.clone_into_attribute(sheltering))
                .to_vec()
        })
    }
}

impl StaticOptions for Sheltering {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Self::Both,
            Self::Physical,
            Self::Magical,
            Self::MagicalCap,
            Self::PhysicalTotal,
            Self::MagicalTotal,
            Self::PhysicalReduction,
            Self::MagicalReduction,
        ]
        .into_iter()
    }
}
