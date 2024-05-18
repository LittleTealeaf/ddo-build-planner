//! Attributes pertaining to defensive stats

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::{chain_tree, enums::StaticOptions, public_modules};

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

public_modules!(health);

/// Attributes related to defenses
#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Defenses {
    Fortification,
    SpellResistance,
    Health(Health),
}

impl Display for Defenses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fortification => write!(f, "Fortification"),
            Self::SpellResistance => write!(f, "Spell Resistance"),
            Self::Health(health) => write!(f, "{health}"),
        }
    }
}

impl ToAttribute for Defenses {
    fn to_attribute(self) -> Attribute {
        Attribute::Defenses(self)
    }
}

impl StaticOptions for Defenses {
    fn get_static() -> impl Iterator<Item = Self> {
        chain_tree!(
            [Self::Fortification, Self::SpellResistance],
            Health::get_static().map(Self::Health)
        )
    }
}

impl CloneBonus for Defenses {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        None
    }
}
