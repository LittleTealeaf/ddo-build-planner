use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, GetBonuses, GetCloned};

use super::{SpellSelector, Tactics};

/// Describes bonuses to difficulty checks of any spell, feat, or ability.
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Enum, Debug, Hash)]
pub enum DifficultyCheck {
    /// Describes basic tactics-based DCs
    Tactics(Tactics),
    /// Describes spell DCs based on any valid spell selector
    Spell(SpellSelector),
}

impl ToString for DifficultyCheck {
    fn to_string(&self) -> String {
        match self {
            DifficultyCheck::Tactics(tactics) => format!("{} DC", tactics.to_string()),
            DifficultyCheck::Spell(spell_selector) => format!("{} DC", spell_selector.to_string()),
        }
    }
}

impl GetBonuses for DifficultyCheck {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        None
    }
}

impl GetCloned<DifficultyCheck> for DifficultyCheck {
    fn get_cloned(&self) -> Option<Vec<DifficultyCheck>> {
        match self {
            DifficultyCheck::Tactics(_) => None,
            DifficultyCheck::Spell(selector) => Some(
                selector
                    .get_cloned()?
                    .into_iter()
                    .map(Self::Spell)
                    .collect(),
            ),
        }
    }
}

impl From<Tactics> for DifficultyCheck {
    fn from(value: Tactics) -> Self {
        Self::Tactics(value)
    }
}

impl From<SpellSelector> for DifficultyCheck {
    fn from(value: SpellSelector) -> Self {
        Self::Spell(value)
    }
}

impl From<DifficultyCheck> for Attribute {
    fn from(value: DifficultyCheck) -> Self {
        Self::DifficultyCheck(value)
    }
}
