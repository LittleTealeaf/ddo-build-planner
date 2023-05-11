use crate::{attribute::Attribute, simple_enum};

simple_enum!(SpellSchool, (Abjuration "Abjuration", Conjuration "Conjuration", Divination "Divintation", Enchantment "Enchantment", Evocation "Evocation", Illusion "Illusion", Necromancy "Necromancy", Transmutation "Transmutation", All "All"));

pub const SPELL_FOCUS_CLONE_ATTRIBUTES: [Attribute; 8] = [
    Attribute::SpellFocus(SpellSchool::Abjuration),
    Attribute::SpellFocus(SpellSchool::Conjuration),
    Attribute::SpellFocus(SpellSchool::Divination),
    Attribute::SpellFocus(SpellSchool::Enchantment),
    Attribute::SpellFocus(SpellSchool::Evocation),
    Attribute::SpellFocus(SpellSchool::Illusion),
    Attribute::SpellFocus(SpellSchool::Necromancy),
    Attribute::SpellFocus(SpellSchool::Transmutation),
];
