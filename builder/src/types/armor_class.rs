//! Armor class types and calculations

use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

/// Represents different attributes that relate to Armor Class
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArmorClass {
    /// Total armor class.
    ///
    /// Do not add bonuses to this. This attribute will be automatically calculated from other
    /// attributes
    Total,
    /// Generic bonuses to Armor Class
    #[serde(rename = "b", alias = "Bonus")]
    Bonus,
    /// Bonuses from the Armor
    ///
    /// The standard type to prevent stacking is [`BonusType::Standard`]
    ///
    /// [`BonusType::Standard`]: crate::bonus::BonusType::Standard
    #[serde(rename = "ab", alias = "ArmorBonus")]
    ArmorBonus,
    /// Bonuses from the Shield
    ///
    /// The standard type to prevent stacking is [`BonusType::Standard`]
    ///
    /// [`BonusType::Standard`]: crate::bonus::BonusType::Standard
    #[serde(rename = "sb", alias = "ShieldBonus")]
    ShieldBonus,
    /// Scalar value for Armor AC Bonus
    #[serde(rename = "as", alias = "ArmorScalar")]
    ArmorScalar,
    /// Scalar value for Shield AC Bonus
    #[serde(rename = "ss", alias = "ShieldScalar")]
    ShieldScalar,
    /// Armor Max Dex Bonus
    #[serde(rename = "amd", alias = "ArmorMaxDex")]
    ArmorMaxDex,
    /// Shield Max Dex Bonus
    #[serde(rename = "smd", alias = "ShieldMaxDex")]
    ShieldMaxDex,
    /// Natural Armor
    #[serde(rename = "n", alias = "NaturalArmor")]
    NaturalArmor,
    /// Total Scalar
    #[serde(rename = "ts", alias = "TotalScalar")]
    TotalScalar,
}

impl Display for ArmorClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Total => write!(f, "Total Armor Class"),
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

impl StaticOptions for ArmorClass {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Self::TotalScalar,
            Self::Total,
            Self::Bonus,
            Self::ArmorBonus,
            Self::ShieldBonus,
            Self::ArmorScalar,
            Self::ShieldScalar,
            Self::ArmorMaxDex,
            Self::ShieldMaxDex,
            Self::NaturalArmor,
        ]
        .into_iter()
    }
}
