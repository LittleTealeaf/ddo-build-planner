use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetCloned},
    player_class::PlayerClass,
};

use super::{SpellPower, SpellSchool, SpellType};

/// Acts as a selector for any type of spell.
///
/// This is used for caster level or spell dc bonuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, Hash)]
pub enum SpellSelector {
    /// All spells that fall under a specific school.
    School(SpellSchool),
    /// All spells that are for a specific player class.
    Class(PlayerClass),
    /// All spells that fall under specific categories
    Type(SpellType),
    /// All spells of a certain element
    SpellPower(SpellPower),
}

impl SpellSelector {
    /// Converts into [`Attribute::SpellDC`]
    pub fn into_spell_dc(self) -> Attribute {
        Attribute::SpellDC(self)
    }
}

impl ToString for SpellSelector {
    fn to_string(&self) -> String {
        match self {
            SpellSelector::School(school) => format!("Spell School: {}", school.to_string()),
            SpellSelector::Class(player_class) => format!("{} Spells", player_class.to_string()),
            SpellSelector::Type(spell_type) => format!("{} Spells", spell_type.to_string()),
            SpellSelector::SpellPower(spell_power) => format!("{} Spells", spell_power.to_string()),
        }
    }
}

impl GetCloned<SpellSelector> for SpellSelector {
    fn get_cloned(&self) -> Option<Vec<SpellSelector>> {
        match self {
            SpellSelector::School(school) => {
                Some(school.get_cloned()?.into_iter().map(Self::School).collect())
            }
            SpellSelector::Type(spell_type) => Some(
                spell_type
                    .to_player_classes()?
                    .into_iter()
                    .map(Self::Class)
                    .collect(),
            ),
            SpellSelector::SpellPower(spell_power) => Some(
                spell_power
                    .get_cloned()?
                    .into_iter()
                    .map(Self::SpellPower)
                    .collect(),
            ),
            _ => None,
        }
    }
}

impl From<SpellSchool> for SpellSelector {
    fn from(value: SpellSchool) -> Self {
        Self::School(value)
    }
}

impl From<PlayerClass> for SpellSelector {
    fn from(value: PlayerClass) -> Self {
        Self::Class(value)
    }
}

impl From<SpellType> for SpellSelector {
    fn from(value: SpellType) -> Self {
        Self::Type(value)
    }
}

impl From<SpellPower> for SpellSelector {
    fn from(value: SpellPower) -> Self {
        Self::SpellPower(value)
    }
}
