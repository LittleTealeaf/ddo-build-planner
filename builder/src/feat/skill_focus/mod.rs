use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
    types::Skill,
};

use super::Feat;

#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SkillFocusFeat {
    SkillFocus(Skill),
}

impl GetBonuses for SkillFocusFeat {
    fn get_bonuses(&self, _: f32) -> Option<Vec<crate::bonus::Bonus>> {
        match self {
            SkillFocusFeat::SkillFocus(skill) => Some(vec![Bonus::new(
                Attribute::Skill(*skill),
                BonusType::Stacking,
                3f32.into(),
                Attribute::Feat(Feat::SkillFocus(*self)).into(),
                None,
            )]),
        }
    }
}

impl Display for SkillFocusFeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillFocusFeat::SkillFocus(skill) => write!(f, "Skill Focus: {skill}"),
        }
    }
}
