use std::fmt::Display;

use serde::{Serialize, Deserialize};


/// Represents different attributes that relate to Armor Class
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArmorClass {
    /// Bonuses to armor class from armor
    ArmorBonus,
    /// Bonuses to armor class from shields
    ShieldBonus,
    /// Scaling for [`ArmorClass::ArmorBonus`]
    ArmorScalar,
    /// Scaling for [`ArmorClass::ShieldBonus`]
    ShieldScalar,
    /// Natural Armor
    NaturalArmor,
    /// Max Dex Bonus for Armor
    ArmorMaxDexBonus,
    /// Max Dex Bonus for Tower Shield
    ShieldMaxDexBonus,
    /// Calculated Max Dex Bonus
    ///
    /// DO NOT MANUALLY ADD BONUSES TO THIS ATTRIBUTE.
    CalculatedMaxDexBonus,
    /// Flat bonuses to armor class
    Bonus,
    /// Scaling for [`ArmorClass::Bonus`]
    Scalar,
}

impl Display for ArmorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bonus => write!(f, "Armor Class"),
            Self::ArmorBonus => write!(f, "Armor AC"),
            Self::ShieldBonus => write!(f, "Shield AC"),
            Self::ArmorScalar => write!(f, "% Armor AC"),
            Self::ShieldScalar => write!(f, "% Shield AC"),
            Self::Scalar => write!(f, "% Armor Class"),
            Self::NaturalArmor => write!(f, "Natural Armor"),
            Self::CalculatedMaxDexBonus => write!(f, "Calculated Max Dex Bonus"),
            Self::ArmorMaxDexBonus => write!(f, "Armor Max Dex Bonus"),
            Self::ShieldMaxDexBonus => write!(f, "Tower Shield Max Dex Bonus"),
        }
    }
}

