use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::bonuses::DamageReduction;

/// Different stats that can be applied to a particular weapon
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
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
    /// Damage Reduction Bypasses that the weapon has
    DamageReductionBypass(DamageReduction),
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
            Self::DamageReductionBypass(dr) => write!(f, "{dr} Bypass"),
            Self::CriticalThreatRange => write!(f, "Critical Threat Range"),
        }
    }
}

impl From<DamageReduction> for WeaponStat {
    fn from(value: DamageReduction) -> Self {
        Self::DamageReductionBypass(value)
    }
}
