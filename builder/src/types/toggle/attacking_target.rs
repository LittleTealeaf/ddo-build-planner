use std::fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::all::AllStatic;

use crate::types::{alignment::Alignment, monster_type::MonsterType};

use super::{ToToggle, Toggle};

/// Indicates that the character is attacking / fighting a certain type
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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

impl ToToggle for AttackingTarget {
    fn to_toggle(self) -> super::Toggle {
        Toggle::Attacking(self)
    }
}

impl AllStatic for AttackingTarget {
    fn all() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Tripped],
            MonsterType::all().map(Self::MonsterType),
            Alignment::all().map(Self::Alignment)
        )
    }
}
