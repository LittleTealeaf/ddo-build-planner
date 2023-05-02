use super::{Attributable, Attribute};

pub enum SavingThrow {
    Reflex,
    Fortitude,
    Will,
    Poison,
    Disease,
    Traps,
    Spell,
    Magic,
    Enchantment,
    Illusion,
    Fear,
    Curse,
}

impl Attributable for SavingThrow {
    fn into_attribute(self) -> super::Attribute {
        Attribute::SavingThrow(self)
    }
}
