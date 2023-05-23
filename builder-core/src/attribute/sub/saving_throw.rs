use crate::{
    attribute::{Attribute, GetCloned},
    simple_enum,
};

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
        if matches!(self, Self::All) {
            Some(vec![Self::Fortitude, Self::Reflex, Self::Will])
        } else {
            None
        }
    }
}

impl From<SavingThrow> for Attribute {
    #[inline(always)]
    fn from(value: SavingThrow) -> Self {
        Self::SavingThrow(value)
    }
}
