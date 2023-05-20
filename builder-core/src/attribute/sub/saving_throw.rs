use crate::{bonus::Bonus, simple_enum, attribute::GetCloned};

simple_enum!(
    SavingThrow, "", (
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

    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}

impl GetCloned<SavingThrow> for SavingThrow {
    fn get_cloned(&self) -> Option<Vec<SavingThrow>> {
        if let Self::All = self {
            Some(vec![Self::Fortitude, Self::Reflex, Self::Will])
        } else {
            None
        }
    }
}
