use serde::{Deserialize, Serialize};

use crate::types::{PlayerClass, Skill};

use super::Feat;

/// Describes requirements that must be satisfied in order for a feat to be taken
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatRequirement {
    /// Requires that a feat is taken
    Feat(Feat),
    /// Requires that the player has a number of levels in a class
    ClassLevel(PlayerClass, u8),
    /// Requires that the player has spent a number of points into a skill
    SkillPoints(Skill, u8),
    /// Requires the player to have obtained a given class level
    BaseAttackBonus(u8),
    /// Requires any of the given requirements
    Any(Vec<FeatRequirement>),
    /// Requires all of the given requirements
    All(Vec<FeatRequirement>),
    /// Negates a requirement
    Not(Box<FeatRequirement>),
}

impl FeatRequirement {
    /// Returns the feat requirement for a logical "none" object
    pub fn none(requirements: Vec<Self>) -> Self {
        Self::Not(Box::new(Self::Any(requirements)))
    }

    /// Returns the feat requirement for a logical "not-all" object
    pub fn not_all(requirements: Vec<Self>) -> Self {
        Self::Not(Box::new(Self::All(requirements)))
    }
}

/// Adds the trait that returns the list of requirements that a feat has.
pub trait GetFeatRequirement {
    /// Returns a list of requirements that a feat has. Returns [`None`] if there are no
    /// requirements
    fn get_feat_requirements(&self) -> Option<FeatRequirement>;
}
