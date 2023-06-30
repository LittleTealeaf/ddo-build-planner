use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, DefaultBonuses, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
};

/// The different spell power types used for spell damage
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellPower {
        /// Splits up each bonus into each of the other bonuses.
        Potency,
        /// Universal adds universal bonuses to each of the other spell powers
        Universal,
    /// Acid Spell Power
    Acid,
    /// Light Spell Power
    Light,
    /// Cold Spell Power
    Cold,
    /// Electric Spell Power
    Electric,
    /// Fire Spell Power
    Fire,
    /// Force Spell Power
    Force,
    /// Negative Spell Power
    Negative,
    /// Poison Spell Power
    Poison,
    /// Positive Spell Power
    Positive,
    /// Repair Spell Power
    Repair,
    /// Rust Spell Power
    Rust,
    /// Sonic Spell Power
    Sonic,
}

impl SpellPower {
    /// All of the spell power types except for [`Potency`] and [`Universal`]
    ///
    /// [`Potency`]: SpellPower::Potency
    /// [`Universal`]: SpellPower::Universal
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

/// 0-sized struct used by [`SpellPower`] to differentiate [`GetBonuses`] for [`Attribute::SpellPower`]
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

/// 0-sized struct used by [`SpellPower`] to differentiate [`GetBonuses`] for [`Attribute::SpellCriticalChance`]
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

/// 0-sized struct used by [`SpellPower`] to differentiate [`GetBonuses`] for [`Attribute::SpellCriticalDamage`]
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

macro_rules! from_skill {
    ($skill: ident, $sp: ident) => {
        Bonus::new(
            $crate::attribute::Attribute::SpellPower(SpellPower::$sp),
            BonusType::Stacking,
            $crate::attribute::Attribute::Skill($crate::attribute::types::Skill::$skill).into(),
            $crate::bonus::BonusSource::Base,
            None,
        )
    };
}

impl DefaultBonuses for SpellPower {
    fn get_default_bonuses() -> Vec<Bonus> {
        vec![
            from_skill!(Heal, Positive),
            from_skill!(Heal, Negative),
            from_skill!(Perform, Sonic),
            from_skill!(Spellcraft, Acid),
            from_skill!(Spellcraft, Cold),
            from_skill!(Spellcraft, Electric),
            from_skill!(Spellcraft, Fire),
            from_skill!(Spellcraft, Force),
            from_skill!(Spellcraft, Light),
            from_skill!(Spellcraft, Poison),
        ]
    }
}

impl TrackAttribute for SpellPower {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::Potency)
    }
}

#[cfg(test)]
mod test {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(SpellPower);

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
