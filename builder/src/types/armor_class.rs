//! Armor class types and calculations

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::all::AllStatic;

use crate::attribute::{Attribute, ToAttribute};

/// Represents different attributes that relate to Armor Class
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArmorClass {
    /// Total armor class.
    ///
    /// Do not add bonuses to this. This attribute will be automatically calculated from other
    /// attributes
    TotalArmorClass,
    /// Generic bonuses to Armor Class
    Bonus,
    /// Bonuses from the Armor
    ///
    /// The standard type to prevent stacking is [`BonusType::Standard`]
    ///
    /// [`BonusType::Standard`]: crate::bonus::BonusType::Standard
    ArmorBonus,
    /// Bonuses from the Shield
    ///
    /// The standard type to prevent stacking is [`BonusType::Standard`]
    ///
    /// [`BonusType::Standard`]: crate::bonus::BonusType::Standard
    ShieldBonus,
    /// Scalar value for Armor AC Bonus
    ArmorScalar,
    /// Scalar value for Shield AC Bonus
    ShieldScalar,
    /// Armor Max Dex Bonus
    ArmorMaxDex,
    /// Shield Max Dex Bonus
    ShieldMaxDex,
    /// Natural Armor
    NaturalArmor,
    /// Total Scalar
    TotalScalar,
}

impl Display for ArmorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TotalArmorClass => write!(f, "Armor Class"),
            Self::Bonus => write!(f, "Armor Class Bonus"),
            Self::ArmorBonus => write!(f, "Armor AC Bonus"),
            Self::ShieldBonus => write!(f, "Shield AC Bonus"),
            Self::ArmorScalar => write!(f, "Armor AC Scalar"),
            Self::ShieldScalar => write!(f, "Shield AC Scalar"),
            Self::ArmorMaxDex => write!(f, "Armor Max Dex Bonus"),
            Self::ShieldMaxDex => write!(f, "Shield Max Dex Bonus"),
            Self::NaturalArmor => write!(f, "Natural Armor"),
            Self::TotalScalar => write!(f, "Armor Class Scalar"),
        }
    }
}

impl ToAttribute for ArmorClass {
    fn to_attribute(self) -> Attribute {
        Attribute::ArmorClass(self)
    }
}

impl AllStatic for ArmorClass {
    fn all() -> impl Iterator<Item = Self> {
        [
            Self::TotalScalar,
            Self::TotalArmorClass,
            Self::Bonus,
            Self::ArmorBonus,
            Self::ShieldBonus,
            Self::ArmorScalar,
            Self::ShieldScalar,
            Self::ArmorMaxDex,
            Self::ShieldMaxDex,
            Self::NaturalArmor
        ].into_iter()
    }
}
