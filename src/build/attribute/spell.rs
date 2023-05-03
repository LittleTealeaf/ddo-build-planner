#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SpellSchool {
    Evocation,
    Illusion,
    Conjuration,
    Abjuration,
    Necromancy,
    Enchantment,
    Transmutation
}


#[derive(PartialEq, Eq, Clone, Hash)]
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
    Universal
}
