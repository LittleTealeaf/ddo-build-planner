use core::fmt::{self, Display};

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::types::damage_type::DamageType;

/// Different stats that can be applied to a particular weapon
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponStat {
    /// Bonus to Attack
    #[serde(rename = "att", alias = "Attack")]
    Attack,
    /// Bonus to Damage
    #[serde(rename = "dam", alias = "Damage")]
    Damage,
    /// Bonus to Critical Attacks
    #[serde(rename = "critatt", alias = "CriticalAttack")]
    CriticalAttack,
    /// Bonus to Critical Damage
    #[serde(rename = "critdam", alias = "CriticalDamage")]
    CriticalDamage,
    /// The critical threat range of the weapon
    #[serde(rename = "range", alias = "CriticalThreatRange")]
    CriticalThreatRange,
    /// The damage multiplier of the weapon on critical hits
    #[serde(rename = "multi", alias = "CriticalMultiplier")]
    CriticalMultiplier,
    /// The damage multiplier of the weapon on critical hits on a 19-20
    #[serde(rename = "vorpalmulti", alias = "CriticalMultiplier1920")]
    CriticalMultiplier1920,
    /// Damage Types
    #[serde(rename = "bypass", alias = "DamageType")]
    DamageType(DamageType),
    /// Weapon-specific damage (damage that is included in the [`WeaponStat::DiceMultiplier`])
    #[serde(rename = "wdam", alias = "WeaponDamage")]
    WeaponDamage,
    /// Dice Multiplier, or W
    #[serde(rename = "W", alias = "DiceMultiplier")]
    DiceMultiplier,
    /// Total final damage dealt
    #[serde(rename = "td", alias = "TotalDamage")]
    TotalDamage,
}

impl Display for WeaponStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attack => write!(f, "Attack"),
            Self::Damage => write!(f, "Damage"),
            Self::CriticalAttack => write!(f, "Critical Attack"),
            Self::CriticalDamage => write!(f, "Critical Damage"),
            Self::CriticalMultiplier => write!(f, "Critical Multiplier"),
            Self::CriticalMultiplier1920 => write!(f, "Critical Multiplier (19-20)"),
            Self::DamageType(damage_type) => write!(f, "{damage_type} Damage"),
            Self::CriticalThreatRange => write!(f, "Critical Threat Range"),
            Self::WeaponDamage => write!(f, "Weapon Damage"),
            Self::DiceMultiplier => write!(f, "Dice Multiplier"),
            Self::TotalDamage => write!(f, "Total Damage"),
        }
    }
}

impl From<DamageType> for WeaponStat {
    fn from(value: DamageType) -> Self {
        Self::DamageType(value)
    }
}

impl StaticValues for WeaponStat {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            [
                Self::Attack,
                Self::Damage,
                Self::CriticalAttack,
                Self::CriticalDamage,
                Self::CriticalThreatRange,
                Self::CriticalMultiplier,
                Self::CriticalMultiplier1920,
                Self::WeaponDamage,
                Self::DiceMultiplier,
                Self::TotalDamage
            ],
            DamageType::values().map(Self::DamageType)
        )
    }
}
