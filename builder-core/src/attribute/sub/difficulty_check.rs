use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, GetBonuses, GetCloned};

use super::{SpellSelector, Tactics};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Enum, Debug, Hash)]
pub enum DifficultyCheck {
    Tactics(Tactics),
    Spell(SpellSelector),
}

impl ToString for DifficultyCheck {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl GetBonuses for DifficultyCheck {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        todo!()
    }
}

impl GetCloned<DifficultyCheck> for DifficultyCheck {
    fn get_cloned(&self) -> Option<Vec<DifficultyCheck>> {
        todo!()
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
