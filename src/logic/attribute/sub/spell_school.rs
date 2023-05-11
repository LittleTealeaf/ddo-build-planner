use crate::{logic::attribute::Attribute, simple_enum};

simple_enum!(SpellSchool, (Abjuration "Abjuration", Conjuration "Conjuration", Divination "Divintation", Enchantment "Enchantment", Evocation "Evocation", Illusion "Illusion", Necromancy "Necromancy", Transmutation "Transmutation", All "All"));

// ub const POTENCY_CLONED_ATTRIBUTES: [SpellPower; 13] = [
//     SpellPower::Acid,
//     SpellPower::Light,
//     SpellPower::Cold,
//     SpellPower::Electric,
//     SpellPower::Evil,
//     SpellPower::Fire,
//     SpellPower::Force,
//     SpellPower::Negative,
//     SpellPower::Poison,
//     SpellPower::Positive,
//     SpellPower::Repair,
//     SpellPower::Rust,
//     SpellPower::Sonic,
// ];

pub const SPELL_SCHOOL_ALL_TO_CLONED_ATTRIUBTES: [Attribute; 8] = [
    Attribute::SpellFocus(SpellSchool::Abjuration),
    Attribute::SpellFocus(SpellSchool::Conjuration),
    Attribute::SpellFocus(SpellSchool::Divination),
    Attribute::SpellFocus(SpellSchool::Enchantment),
    Attribute::SpellFocus(SpellSchool::Evocation),
    Attribute::SpellFocus(SpellSchool::Illusion),
    Attribute::SpellFocus(SpellSchool::Necromancy),
    Attribute::SpellFocus(SpellSchool::Transmutation),
];
