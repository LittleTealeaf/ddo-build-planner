use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetCloned},
    simple_enum,
    utils::AsString,
};

use super::DamageReduction;

#[derive(Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize, Hash, Debug)]
/// Represents any stat that a particular weapon can have.
///
/// When in attributes, a [`WeaponHand`] is also included to differentiate between main and off
/// hands, or bonuses that apply to both hands.
pub enum WeaponStat {
    /// Bonus to your to-hit roll whenever you attack.
    Attack,
    /// Bonus to your weapon damage.
    Damage,
    /// Bonus to attack when confirming critical hits
    CriticalAttack,
    /// Bonus to damage on critical hits
    CriticalDamage,
    /// The multipler used when critically hitting
    CriticalMultiplier,
    /// The multiplier used when critically hitting on a roll of a 19-20
    CriticalMultiplier1920,
    /// Ability to bypass damage reduction of a particular type
    DamageReductionBypass(DamageReduction),
}

impl ToString for WeaponStat {
    fn to_string(&self) -> String {
        match self {
            WeaponStat::Attack => String::from("Attack"),
            WeaponStat::Damage => String::from("Damage"),
            WeaponStat::CriticalAttack => String::from("Critical Attack"),
            WeaponStat::CriticalDamage => String::from("Critical Damage"),
            WeaponStat::CriticalMultiplier => String::from("Critical Multiplier"),
            WeaponStat::CriticalMultiplier1920 => String::from("Critical Multipler (19-20)"),
            WeaponStat::DamageReductionBypass(dr) => format!("{} Bypass", dr.to_string()),
        }
    }
}

impl AsString for (&WeaponHand, &WeaponStat) {
    fn as_string(&self) -> String {
        match self {
            (WeaponHand::Both, stat) => stat.to_string(),
            (WeaponHand::Main, stat) => format!("Main Hand {}", stat.to_string()),
            (WeaponHand::Off, stat) => format!("Off Hand {}", stat.to_string()),
        }
    }
}

simple_enum!(WeaponHand, "", (Main "Main Hand", Off "Off Hand", Both "Both Hand"));

impl WeaponHand {
    /// All weapon hands except for [`WeaponHand::Both`]
    pub const VALUES: [WeaponHand; 2] = [WeaponHand::Main, WeaponHand::Off];
}

impl GetCloned<(WeaponHand, WeaponStat)> for (WeaponHand, WeaponStat) {
    fn get_cloned(&self) -> Option<Vec<(WeaponHand, WeaponStat)>> {
        match self {
            (WeaponHand::Both, stat) => {
                Some(vec![(WeaponHand::Main, *stat), (WeaponHand::Off, *stat)])
            }
            (hand, WeaponStat::CriticalMultiplier) => {
                Some(vec![(*hand, WeaponStat::CriticalMultiplier1920)])
            }
            _ => None,
        }
    }
}

impl From<(WeaponHand, WeaponStat)> for Attribute {
    #[inline(always)]
    fn from(value: (WeaponHand, WeaponStat)) -> Self {
        let (hand, stat) = value;
        Self::WeaponStat(hand, stat)
    }
}
