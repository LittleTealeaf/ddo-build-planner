use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::DamageReduction;

/// Different stats that can be applied to a particular weapon
#[cfg_attr(test, derive(enum_map::Enum))]
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
            WeaponStat::Attack => write!(f, "Attack"),
            WeaponStat::Damage => write!(f, "Damage"),
            WeaponStat::CriticalAttack => write!(f, "Critical Attack"),
            WeaponStat::CriticalDamage => write!(f, "Critical Damage"),
            WeaponStat::CriticalMultiplier => write!(f, "Critical Multiplier"),
            WeaponStat::CriticalMultiplier1920 => write!(f, "Critical Multiplier (19-20)"),
            WeaponStat::DamageReductionBypass(dr) => write!(f, "{} Bypass", dr),
            WeaponStat::CriticalThreatRange => write!(f, "Critical Threat Range"),
        }
    }
}

impl From<DamageReduction> for WeaponStat {
    fn from(value: DamageReduction) -> Self {
        Self::DamageReductionBypass(value)
    }
}
