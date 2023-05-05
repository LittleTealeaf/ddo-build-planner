#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum SpellSchool {
    Evocation,
    Illusion,
    Conjuration,
    Abjuration,
    Necromancy,
    Enchantment,
    Transmutation,
}

impl ToString for SpellSchool {
    fn to_string(&self) -> String {
        String::from(match self {
            SpellSchool::Evocation => "Evocation",
            SpellSchool::Illusion => "Illusion",
            SpellSchool::Conjuration => "Conjuration",
            SpellSchool::Abjuration => "Abjuration",
            SpellSchool::Necromancy => "Necromancy",
            SpellSchool::Enchantment => "Enchantment",
            SpellSchool::Transmutation => "Transmutation",
        })
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Copy)]
pub enum SpellDamageType {
    Acid,
    Cold,
    Electric,
    Fire,
    Force,
    Light,
    Negative,
    Poison,
    Positive,
    Repair,
    Sonic,
    Universal,
}
