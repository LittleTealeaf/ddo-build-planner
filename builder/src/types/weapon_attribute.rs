//! weapon Attributes
public_modules!(weapon_hand, weapon_stat);

use core::fmt::{self, Display};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::{enums::StaticOptions, public_modules};

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::{Bonus, BonusTemplate, BonusType, CloneBonus},
};

/// A `WeaponStat` that is specifically for a weapon hand.
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WeaponAttribute(WeaponHand, WeaponStat);

impl Display for WeaponAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(hand, stat) = self;
        write!(f, "{hand}-Hand {stat}")
    }
}

impl From<(WeaponHand, WeaponStat)> for WeaponAttribute {
    fn from((hand, stat): (WeaponHand, WeaponStat)) -> Self {
        Self(hand, stat)
    }
}

impl From<(WeaponHand, WeaponStat)> for Attribute {
    fn from(value: (WeaponHand, WeaponStat)) -> Self {
        WeaponAttribute::from(value).into()
    }
}

impl GetBonuses for WeaponAttribute {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self(hand, WeaponStat::CriticalMultiplier) => Some(vec![BonusTemplate::new(
                (*hand, WeaponStat::CriticalMultiplier1920),
                BonusType::Stacking,
                value,
                None,
            )]),
            _ => None,
        }
    }
}

impl CloneBonus for WeaponAttribute {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        let Self(hand, stat) = self;
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
                WeaponStat::DamageType(dr) => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::DamageType(*dr)))
                }
                WeaponStat::CriticalThreatRange => {
                    WeaponHand::VALUES.map(|hand| (hand, WeaponStat::CriticalThreatRange))
                }
            }
            .map(|stat| {
                Bonus::new(
                    stat,
                    *bonus.bonus_type(),
                    bonus.value().clone(),
                    bonus.source().clone(),
                    bonus.condition().cloned(),
                )
            })
            .to_vec()
        })
    }
}

impl ToAttribute for WeaponAttribute {
    fn to_attribute(self) -> Attribute {
        Attribute::Weapon(self)
    }
}

impl StaticOptions for WeaponAttribute {
    fn get_static() -> impl Iterator<Item = Self> {
        WeaponHand::get_static()
            .flat_map(|hand| WeaponStat::get_static().map(move |stat| Self(hand, stat)))
    }
}
