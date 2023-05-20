use crate::{player_class::PlayerClass, simple_enum, attribute::GetCloned};

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

impl SpellType {
    /// Converts the spell type to each of the player classes represented by that type.
    pub fn to_player_classes(&self) -> Option<Vec<PlayerClass>> {
        match self {
            Self::Arcane => Some(vec![
                PlayerClass::Wizard,
                PlayerClass::Sorcerer,
                PlayerClass::Ranger,
                PlayerClass::DarkHunter,
            ]),
            _ => None,
        }
    }
}

impl GetCloned<CasterLevel> for CasterLevel {
    fn get_cloned(&self) -> Option<Vec<CasterLevel>> {
        match self {
            Self::SpellType(spell_type) => Some(
                spell_type
                    .to_player_classes()?
                    .into_iter()
                    .map(Self::PlayerClass)
                    .collect(),
            ),
            Self::SpellPower(spell_power) => Some(
                spell_power
                    .get_cloned()?
                    .into_iter()
                    .map(Self::SpellPower)
                    .collect(),
            ),
            Self::SpellSchool(school) => Some(
                school
                    .get_cloned()?
                    .into_iter()
                    .map(Self::SpellSchool)
                    .collect(),
            ),
            _ => None,
        }
    }
}
