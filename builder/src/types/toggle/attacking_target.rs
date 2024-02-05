use std::fmt::{self, Display};

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl StaticOptions for AttackingTarget {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Tripped],
            MonsterType::get_static().map(Self::MonsterType),
            Alignment::get_static().map(Self::Alignment)
        )
    }
}
