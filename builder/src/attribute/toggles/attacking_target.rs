use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::types::{Alignment, MonsterType};

/// Indicates that the character is attacking / fighting a certain type
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttackingTarget {
    /// The enemy is tripped
    Tripped,
    /// The enemy is a certain monster tyype
    MonsterType(MonsterType),
    /// The enemy is a certain alignment
    Alignment(Alignment),
}

impl Display for AttackingTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackingTarget::Tripped => write!(f, "Tripped Target"),
            AttackingTarget::MonsterType(monster_type) => monster_type.fmt(f),
            AttackingTarget::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
