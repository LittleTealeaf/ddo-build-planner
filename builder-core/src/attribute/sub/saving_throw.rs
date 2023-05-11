use crate::{

        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType},

    simple_enum,
};

simple_enum!(SavingThrow, (Fortitude "Fortitude", Poison "Poison", Disease "Disease", Reflex "Reflex", Traps "Traps", Spell "Spell", Magic "Magic", Will "Will", Enchantment "Enchantment", Illusion "Illusion", Fear "Fear", Curse "Curse", All "All"));

macro_rules! child_saving_throw {
    ($parent: ident, $child: ident, $value: expr) => {
        Bonus::new(
            Attribute::SavingThrow(SavingThrow::$child),
            BonusType::Stacking,
            $value,
            BonusSource::Attribute(Attribute::SavingThrow(SavingThrow::$parent)),
            None,
        )
    };
}

impl SavingThrow {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            SavingThrow::Fortitude => Some(vec![
                child_saving_throw!(Fortitude, Poison, value),
                child_saving_throw!(Fortitude, Disease, value),
            ]),
            SavingThrow::Reflex => Some(vec![
                child_saving_throw!(Reflex, Traps, value),
                child_saving_throw!(Reflex, Spell, value),
                child_saving_throw!(Reflex, Magic, value),
            ]),
            SavingThrow::Will => Some(vec![
                child_saving_throw!(Will, Enchantment, value),
                child_saving_throw!(Will, Illusion, value),
                child_saving_throw!(Will, Fear, value),
                child_saving_throw!(Will, Curse, value),
            ]),
            _ => None,
        }
    }
}

impl From<SavingThrow> for Attribute {
    fn from(value: SavingThrow) -> Self {
        Attribute::SavingThrow(value)
    }
}
