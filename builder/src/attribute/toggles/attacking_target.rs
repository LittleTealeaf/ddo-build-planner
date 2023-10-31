use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::types::{Alignment, MonsterType};

/// Indicates that the character is attacking / fighting a certain type
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
            Self::Tripped => write!(f, "Tripped Target"),
            Self::MonsterType(monster_type) => monster_type.fmt(f),
            Self::Alignment(alignment) => alignment.fmt(f),
        }
    }
}
