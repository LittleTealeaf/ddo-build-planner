use crate::{
    attribute::{Attribute, DefaultBonuses, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
    types::SpellPower,
};

/// 0-sized struct used by [`SpellPower`] to differentiate [`GetBonuses`] for [`Attribute::SpellPower`]
pub struct _SpellPower;

impl GetBonuses<_SpellPower> for SpellPower {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        matches!(self, Self::Universal).then(|| {
            Self::SPELL_POWERS
                .map(|sp| {
                    Bonus::new(
                        Attribute::SpellPower(sp),
                        BonusType::Stacking,
                        value.into(),
                        Attribute::SpellPower(Self::Universal).into(),
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
            Self::SPELL_POWERS
                .map(|sp| {
                    Bonus::new(
                        Attribute::SpellCriticalChance(sp),
                        BonusType::Stacking,
                        value.into(),
                        Attribute::SpellCriticalChance(Self::Universal).into(),
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
            Self::SPELL_POWERS
                .map(|sp| {
                    Bonus::new(
                        Attribute::SpellCriticalDamage(sp),
                        BonusType::Stacking,
                        value.into(),
                        Attribute::SpellCriticalDamage(Self::Universal).into(),
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
                Attribute::SpellPower(Self::Potency) => {
                    Some(Self::SPELL_POWERS.map(Attribute::SpellPower))
                }
                Attribute::SpellCriticalChance(Self::Potency) => {
                    Some(Self::SPELL_POWERS.map(Attribute::SpellCriticalChance))
                }
                Attribute::SpellCriticalDamage(Self::Potency) => {
                    Some(Self::SPELL_POWERS.map(Attribute::SpellCriticalDamage))
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
            $crate::attribute::Attribute::SpellPower($crate::types::SpellPower::from(
                $crate::types::DamageType::$sp,
            )),
            BonusType::Stacking,
            $crate::attribute::Attribute::Skill($crate::types::Skill::$skill).into(),
            $crate::bonus::BonusSource::Base,
            None,
        )
    };
}

impl DefaultBonuses for SpellPower {
    type Iterator = [Bonus; 10];

    fn get_default_bonuses() -> Self::Iterator {
        [
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
mod tests {
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
        for sp in SpellPower::SPELL_POWERS {
            assert!(sp.is_tracked());
            assert!(Attribute::SpellPower(sp).is_tracked());
            assert!(Attribute::SpellCriticalChance(sp).is_tracked());
            assert!(Attribute::SpellCriticalDamage(sp).is_tracked());
        }
    }
}

