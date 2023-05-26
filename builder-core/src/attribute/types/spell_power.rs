use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
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
