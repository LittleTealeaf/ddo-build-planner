use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::DamageType;

/// Different stats that can be applied to a particular weapon
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponStat {
    /// Bonus to Attack
    Attack,
    /// Bonus to Damage
    Damage,
    /// Bonus to Critical Attacks
    CriticalAttack,
    /// Bonus to Critical Damage
    CriticalDamage,
    /// The critical threat range of the weapon
    CriticalThreatRange,
    /// The damage multiplier of the weapon on critical hits
    CriticalMultiplier,
    /// THe damage multiplier of the weapon on critical hits on a 19-20
    CriticalMultiplier1920,
    /// Damage Types
    DamageType(DamageType),
}

impl Display for WeaponStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attack => write!(f, "Attack"),
            Self::Damage => write!(f, "Damage"),
            Self::CriticalAttack => write!(f, "Critical Attack"),
            Self::CriticalDamage => write!(f, "Critical Damage"),
            Self::CriticalMultiplier => write!(f, "Critical Multiplier"),
            Self::CriticalMultiplier1920 => write!(f, "Critical Multiplier (19-20)"),
            Self::DamageType(damage_type) => write!(f, "{damage_type} Damage"),
            Self::CriticalThreatRange => write!(f, "Critical Threat Range"),
        }
    }
}

impl From<DamageType> for WeaponStat {
    fn from(value: DamageType) -> Self {
        Self::DamageType(value)
    }
}
