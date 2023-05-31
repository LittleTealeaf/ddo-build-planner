use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum SpellPower {
    Acid,
    Light,
    Cold,
    Electric,
    Fire,
    Force,
    Negative,
    Poison,
    Positive,
    Repair,
    Rust,
    Sonic,
    Potency,
    Universal,
}

impl SpellPower {
    pub const ALL: [Self; 12] = [
        Self::Acid,
        Self::Light,
        Self::Cold,
        Self::Electric,
        Self::Fire,
        Self::Force,
        Self::Negative,
        Self::Poison,
        Self::Positive,
        Self::Repair,
        Self::Rust,
        Self::Sonic,
    ];
}

impl Display for SpellPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellPower::Acid => write!(f, "Acid"),
            SpellPower::Light => write!(f, "Light"),
            SpellPower::Cold => write!(f, "Cold"),
            SpellPower::Electric => write!(f, "Electric"),
            SpellPower::Fire => write!(f, "Fire"),
            SpellPower::Force => write!(f, "Force"),
            SpellPower::Negative => write!(f, "Negative"),
            SpellPower::Poison => write!(f, "Poison"),
            SpellPower::Positive => write!(f, "Positive"),
            SpellPower::Repair => write!(f, "Repair"),
            SpellPower::Rust => write!(f, "Rust"),
            SpellPower::Sonic => write!(f, "Sonic"),
            SpellPower::Universal => write!(f, "Universal"),
            SpellPower::Potency => write!(f, "Potency"),
        }
    }
}

pub struct _SpellPower;

impl GetBonuses<_SpellPower> for SpellPower {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        matches!(self, Self::Universal).then(|| {
            SpellPower::ALL
                .map(|sp| {
                    Bonus::new(
                        Attribute::SpellPower(sp),
                        BonusType::Stacking,
                        value.into(),
                        Attribute::SpellPower(SpellPower::Universal).into(),
                        None,
                    )
                })
                .into()
        })
    }
}

pub struct _SpellCriticalChance;

impl GetBonuses<_SpellCriticalChance> for SpellPower {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        matches!(self, Self::Universal).then(|| {
            SpellPower::ALL
                .map(|sp| {
                    Bonus::new(
                        Attribute::SpellCriticalChance(sp),
                        BonusType::Stacking,
                        value.into(),
                        Attribute::SpellCriticalChance(SpellPower::Universal).into(),
                        None,
                    )
                })
                .into()
        })
    }
}

pub struct _SpellCriticalDamage;

impl GetBonuses<_SpellCriticalDamage> for SpellPower {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        matches!(self, Self::Universal).then(|| {
            SpellPower::ALL
                .map(|sp| {
                    Bonus::new(
                        Attribute::SpellCriticalDamage(sp),
                        BonusType::Stacking,
                        value.into(),
                        Attribute::SpellCriticalDamage(SpellPower::Universal).into(),
                        None,
                    )
                })
                .into()
        })
    }
}

impl CloneBonus for SpellPower {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        Some(
            match bonus.get_attribute() {
                Attribute::SpellPower(SpellPower::Potency) => {
                    Some(Self::ALL.map(Attribute::SpellPower))
                }
                Attribute::SpellCriticalChance(SpellPower::Potency) => {
                    Some(Self::ALL.map(Attribute::SpellCriticalChance))
                }
                Attribute::SpellCriticalDamage(SpellPower::Potency) => {
                    Some(Self::ALL.map(Attribute::SpellCriticalDamage))
                }
                _ => None,
            }?
            .map(|attribute| {
                Bonus::new(
                    attribute,
                    bonus.get_type(),
                    bonus.get_value(),
                    bonus.get_source(),
                    bonus.get_condition(),
                )
            })
            .to_vec(),
        )
    }
}

impl TrackAttribute for SpellPower {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::Potency)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn potency_is_not_tracked() {
        assert!(!SpellPower::Potency.is_tracked());
        assert!(!Attribute::SpellPower(SpellPower::Potency).is_tracked());
        assert!(!Attribute::SpellCriticalChance(SpellPower::Potency).is_tracked());
        assert!(!Attribute::SpellCriticalDamage(SpellPower::Potency).is_tracked());
    }

    #[test]
    fn spell_powers_are_tracked() {
        for sp in SpellPower::ALL {
            assert!(sp.is_tracked());
            assert!(Attribute::SpellPower(sp).is_tracked());
            assert!(Attribute::SpellCriticalChance(sp).is_tracked());
            assert!(Attribute::SpellCriticalDamage(sp).is_tracked());
        }
    }
}
