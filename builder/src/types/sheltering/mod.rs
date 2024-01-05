//! Physical and Magical Sheltering
public_modules!(bonuses);

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::public_modules;

use crate::attribute::{Attribute, ToAttribute};

/// Sheltering attributes grant a % reduction to damage from that type.
///
/// Magical Sheltering can be capped at a certain amount based on equipment and enhancements, which is tracked with [`Sheltering::MagicalCap`]
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn to_attribute(self) -> crate::attribute::Attribute {
        Attribute::Sheltering(self)
    }
}
