use crate::{simple_enum, attribute::Attribute};

pub const EXPORT_SHELTERING_ATTRIBUTES: [Attribute; 2] = [
    Attribute::Defensive(Defensive::PhysicalSheltering),
    Attribute::Defensive(Defensive::MagicalSheltering)
];

simple_enum!(Defensive, (PhysicalSheltering "Physical Sheltering", MagicalSheltering "Magical Sheltering", MagicalShelteringCap "Magical Sheltering Cap", Sheltering "Sheltering", MissileDeflection "Missile Deflection"));
