use crate::{attribute::Attribute, player_class::PlayerClass, simple_enum};

use super::{SpellPower, SpellSchool};

simple_enum!(
    CasterLevel,
    "", 
    (
        SpellPower(spellpower: SpellPower) format!("{} Spell Caster Level", spellpower.to_string()),
        PlayerClass(playerclass: PlayerClass) format!("{} Spell Caster Level", playerclass.to_string()),
        SpellType(spelltype: SpellType) format!("{} Spell Caster Level", spelltype.to_string()),
        SpellSchool(spellschool: SpellSchool) format!("{} Spell Caster Level", spellschool.to_string())
    )
);

// TODO: Add Per Spell

simple_enum!(
    SpellType,
    "",
    (
        Arcane "Arcane", Divine "Divine"
    )
);

impl CasterLevel {
    pub fn get_cloned_attributes(&self) -> Option<Vec<CasterLevel>> {
        match self {
            Self::SpellType(SpellType::Arcane) => Some(vec![
                Self::PlayerClass(PlayerClass::Wizard),
                Self::PlayerClass(PlayerClass::Sorcerer),
                Self::PlayerClass(PlayerClass::Ranger),
                Self::PlayerClass(PlayerClass::DarkHunter),
            ]),
            _ => None,
        }
    }
}
