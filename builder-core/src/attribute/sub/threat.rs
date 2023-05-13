use crate::simple_enum;


pub const ALL_THREAT: [Threat; 3] = [
    Threat::Spell,
    Threat::Ranged,
    Threat::Melee
];

simple_enum!(Threat, (Spell "Spell", Ranged "Ranged", Melee "Melee", All "All"));
