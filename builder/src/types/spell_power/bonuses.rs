use crate::{
    attribute::Attribute,
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
            .map(|attribute| bonus.clone_with_attribute(attribute))
            .to_vec(),
        )
    }
}
