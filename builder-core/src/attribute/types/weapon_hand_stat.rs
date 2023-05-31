use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
};

use super::{WeaponHand, WeaponStat};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Enum)]
pub struct WeaponHandStat(WeaponHand, WeaponStat);

impl Display for WeaponHandStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let WeaponHandStat(hand, stat) = self;
        write!(f, "{}-Hand {}", hand, stat)
    }
}

impl From<(WeaponHand, WeaponStat)> for WeaponHandStat {
    fn from((hand, stat): (WeaponHand, WeaponStat)) -> Self {
        Self(hand, stat)
    }
}

impl From<(WeaponHand, WeaponStat)> for Attribute {
    fn from(value: (WeaponHand, WeaponStat)) -> Self {
        WeaponHandStat::from(value).into()
    }
}

impl From<WeaponHandStat> for Attribute {
    fn from(value: WeaponHandStat) -> Self {
        Attribute::Weapon(value)
    }
}

impl GetBonuses for WeaponHandStat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            WeaponHandStat(hand, WeaponStat::CriticalMultiplier) => Some(vec![Bonus::new(
                (*hand, WeaponStat::CriticalMultiplier1920).into(),
                BonusType::Stacking,
                value.into(),
                Attribute::from((*hand, WeaponStat::CriticalMultiplier)).into(),
                None,
            )]),
            _ => None,
        }
    }
}

impl TrackAttribute for WeaponHandStat {
    fn is_tracked(&self) -> bool {
        let WeaponHandStat(hand, _) = self;
        !matches!(hand, WeaponHand::Both)
    }
}

impl CloneBonus for WeaponHandStat {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        let WeaponHandStat(hand, stat) = self;
        matches!(hand, WeaponHand::Both).then(|| {
            match stat {
                WeaponStat::Attack => WeaponHand::VALUES.map(|hand| (hand, WeaponStat::Attack)),
                WeaponStat::Damage => WeaponHand::VALUES.map(|hand| (hand, WeaponStat::Damage)),
                WeaponStat::CriticalAttack => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::CriticalAttack))
                }
                WeaponStat::CriticalDamage => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::CriticalDamage))
                }
                WeaponStat::CriticalMultiplier => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::CriticalMultiplier))
                }
                WeaponStat::CriticalMultiplier1920 => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::CriticalMultiplier1920))
                }
                WeaponStat::DamageReductionBypass(dr) => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::DamageReductionBypass(*dr)))
                }
            }
            .map(|stat| {
                Bonus::new(
                    stat.into(),
                    bonus.get_type(),
                    bonus.get_value(),
                    bonus.get_source(),
                    bonus.get_condition(),
                )
            })
            .to_vec()
        })
    }
}
