use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
    types::{WeaponHand, WeaponStat},
};

/// A `WeaponStat` that is specifically for a weapon hand.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WeaponAttribute(WeaponHand, WeaponStat);

impl Display for WeaponAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self(hand, WeaponStat::CriticalMultiplier) => Some(vec![Bonus::new(
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

impl TrackAttribute for WeaponAttribute {
    fn is_tracked(&self) -> bool {
        let Self(hand, _) = self;
        !matches!(hand, WeaponHand::Both)
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

#[cfg(test)]
mod tests {
    use super::*;

    use enum_map::Enum;

    #[test]
    fn both_hands_is_not_tracked() {
        for i in 0..WeaponStat::LENGTH {
            let stat = WeaponStat::from_usize(i);
            let hand_stat = WeaponAttribute::from((WeaponHand::Both, stat));
            assert!(!hand_stat.is_tracked());
            assert!(!Attribute::from(hand_stat).is_tracked());
        }
    }

    #[test]
    fn either_hand_is_tracked() {
        for i in 0..WeaponHand::LENGTH {
            for hand in [WeaponHand::Off, WeaponHand::Main] {
                let hand_stat = WeaponAttribute::from((hand, WeaponStat::from_usize(i)));
                assert!(hand_stat.is_tracked());
                assert!(Attribute::from(hand_stat).is_tracked());
            }
        }
    }
}
