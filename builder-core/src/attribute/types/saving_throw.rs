use std::fmt::Display;

use enum_map::Enum;

use crate::attribute::Attribute;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum SavingThrow {
    Fortitude,
    Poison,
    Disease,
    Reflex,
    Traps,
    Spell,
    Magic,
    Will,
    Enchantment,
    Illusion,
    Fear,
    Curse,
}

impl SavingThrow {
    pub const ALL: [Self; 3] = [Self::Fortitude, Self::Reflex, Self::Will];
}

impl Display for SavingThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SavingThrow::Fortitude => write!(f, "Fortitude"),
            SavingThrow::Poison => write!(f, "Poison"),
            SavingThrow::Disease => write!(f, "Disease"),
            SavingThrow::Reflex => write!(f, "Reflex"),
            SavingThrow::Traps => write!(f, "Traps"),
            SavingThrow::Spell => write!(f, "Spell"),
            SavingThrow::Magic => write!(f, "Magic"),
            SavingThrow::Will => write!(f, "Will"),
            SavingThrow::Enchantment => write!(f, "Enchantment"),
            SavingThrow::Illusion => write!(f, "Illusion"),
            SavingThrow::Fear => write!(f, "Fear"),
            SavingThrow::Curse => write!(f, "Curse"),
        }
    }
}

impl From<SavingThrow> for Attribute {
    fn from(value: SavingThrow) -> Self {
        Attribute::SavingThrow(value)
    }
}
