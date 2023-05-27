use std::fmt::Display;

use enum_map::Enum;

use super::DamageReduction;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Enum)]
pub enum WeaponStat {
    Attack,
    Damage,
    CriticalAttack,
    CriticalDamage,
    CriticalMultiplier,
    CriticalMultiplier1920,
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
        }
    }
}

impl From<DamageReduction> for WeaponStat {
    fn from(value: DamageReduction) -> Self {
        Self::DamageReductionBypass(value)
    }
}
