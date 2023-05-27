use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
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
