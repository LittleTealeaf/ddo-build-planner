use crate::{attribute_subtype, build::attribute::Attribute};

attribute_subtype!(SavingThrow, (Fortitude "Fortitude"), (Reflex "Reflex"), (Will "Will"), (Spells "Spells"), (Traps "Traps"), (Fear "Fear"), (Enchantment "Enchantment"), (Curses "Curses"), (Illusions "Illusions"), (Sleep "Sleep"), (Diseases "Diseases"), (Exhaustion "Exhaustion"), (Nausea "Nausea"), (Paralysis "Paralysis"), (Poison "Poison"));

impl From<SavingThrow> for Attribute {
    fn from(value: SavingThrow) -> Self {
        Attribute::SavingThrow(value)
    }
}
