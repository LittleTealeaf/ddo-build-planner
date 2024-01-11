use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, CloneBonus},
    types::spell_power::SpellPower,
};

impl CloneBonus for SpellPower {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        Some(
            match bonus.attribute() {
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
            .map(|attribute| bonus.clone_into_attribute(attribute))
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
mod tests {

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
        for sp in SpellPower::SPELL_POWERS {
            assert!(sp.is_tracked());
            assert!(Attribute::SpellPower(sp).is_tracked());
            assert!(Attribute::SpellCriticalChance(sp).is_tracked());
            assert!(Attribute::SpellCriticalDamage(sp).is_tracked());
        }
    }
}
