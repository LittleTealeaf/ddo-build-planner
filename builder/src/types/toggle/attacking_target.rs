use core::fmt::{self, Display};

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::types::{alignment::Alignment, monster_type::MonsterType};

use super::{ToToggle, Toggle};

/// Indicates that the character is attacking / fighting a certain type
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttackingTarget {
    /// The enemy is tripped
    Tripped,
    /// Helpless
    Helpless,
    /// The enemy is a certain monster type
    MonsterType(MonsterType),
    /// The enemy is a certain alignment
    Alignment(Alignment),
}

impl Display for AttackingTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Helpless => write!(f, "Helpless Target"),
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

impl StaticValues for AttackingTarget {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Tripped],
            MonsterType::values().map(Self::MonsterType),
            Alignment::values().map(Self::Alignment)
        )
    }
}
