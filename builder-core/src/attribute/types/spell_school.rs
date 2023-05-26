use std::fmt::Display;

use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl SpellSchool {
    pub const ALL: [Self; 8] = [
        Self::Abjuration,
        Self::Conjuration,
        Self::Divination,
        Self::Enchantment,
        Self::Evocation,
        Self::Illusion,
        Self::Necromancy,
        Self::Transmutation,
    ];
}

impl Display for SpellSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpellSchool::Abjuration => write!(f, "Abjuration"),
            SpellSchool::Conjuration => write!(f, "Conjuration"),
            SpellSchool::Divination => write!(f, "Divination"),
            SpellSchool::Enchantment => write!(f, "Enchantment"),
            SpellSchool::Evocation => write!(f, "Evocation"),
            SpellSchool::Illusion => write!(f, "Illusion"),
            SpellSchool::Necromancy => write!(f, "Necromancy"),
            SpellSchool::Transmutation => write!(f, "Transmutation"),
        }
    }
}
