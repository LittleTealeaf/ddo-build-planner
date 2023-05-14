use crate::{bonus::Bonus, simple_enum};

simple_enum!(
    SavingThrow, (
        Fortitude "Fortitude",
        Poison "Poison",
        Disease "Disease",
        Reflex "Reflex",
        Traps "Traps",
        Spell "Spell",
        Magic "Magic",
        Will "Will",
        Enchantment "Enchantment",
        Illusion "Illusion",
        Fear "Fear",
        Curse "Curse",
        All "All"
    )
);

macro_rules! child_saving_throw {
    ($parent: ident, $child: ident, $value: expr) => {
        $crate::bonus::Bonus::new(
            $crate::attribute::Attribute::SavingThrow(SavingThrow::$child),
            $crate::bonus::BonusType::Stacking,
            $value,
            $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::SavingThrow(
                SavingThrow::$parent,
            )),
            None,
        )
    };
}

impl SavingThrow {
    pub fn get_cloned_values(&self) -> Option<Vec<Self>> {
        if let Self::All = self {
            Some(vec![Self::Fortitude, Self::Reflex, Self::Will])
        } else {
            None
        }
    }

    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            SavingThrow::Fortitude => Some(vec![
                child_saving_throw!(Fortitude, Poison, value),
                child_saving_throw!(Fortitude, Disease, value),
            ]),
            _ => None,
        }
    }
}
