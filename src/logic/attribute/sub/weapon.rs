use crate::{logic::bonus::BonusSource, simple_attribute_enum};

use super::DamageReduction;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponStat {
    Attack,
    Damage,
    CriticalAttack,
    CriticalDamage,
    CriticalThreat,
    CriticalMultiplier,
    CriticalMultiplier1920,
    DamageReductionBypass(DamageReduction),
}

impl ToString for WeaponStat {
    fn to_string(&self) -> String {
        match self {
            WeaponStat::Attack => String::from("Attack"),
            WeaponStat::Damage => String::from("Damage"),
            WeaponStat::CriticalAttack => String::from("Critical Attack"),
            WeaponStat::CriticalDamage => String::from("Critical Damage"),
            WeaponStat::CriticalThreat => String::from("Critical Threat"),
            WeaponStat::CriticalMultiplier => String::from("Critical Multiplier"),
            WeaponStat::CriticalMultiplier1920 => String::from("Critical Multiplier (19-20)"),
            WeaponStat::DamageReductionBypass(bypass) => {
                format!("{} Damage Reduction Bypass", bypass.to_string())
            }
        }
    }
}

simple_attribute_enum!(WeaponHand, (Both "", MainHand "Main Hand ", OffHand "Off Hand "));
