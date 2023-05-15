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
        All "All Saving Throws"
    )
);

impl SavingThrow {
    pub fn get_cloned_values(&self) -> Option<Vec<Self>> {
        if let Self::All = self {
            Some(vec![Self::Fortitude, Self::Reflex, Self::Will])
        } else {
            None
        }
    }

    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}
