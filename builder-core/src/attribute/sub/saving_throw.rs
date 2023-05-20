use crate::{simple_enum, attribute::{GetCloned, Attribute}};

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

impl GetCloned<SavingThrow> for SavingThrow {
    fn get_cloned(&self) -> Option<Vec<SavingThrow>> {
        if let Self::All = self {
            Some(vec![Self::Fortitude, Self::Reflex, Self::Will])
        } else {
            None
        }
    }
}

impl From<SavingThrow> for Attribute {
    fn from(value: SavingThrow) -> Self {
        Self::SavingThrow(value)
    }
}
