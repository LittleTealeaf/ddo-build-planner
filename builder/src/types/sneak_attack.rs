//! Sneak Attack Type

use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

/// Bonuses to attack and damage when sneak attacking
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SneakAttack {
    /// Bonuses to attack
    #[serde(rename = "a", alias = "Attack")]
    Attack,
    /// Bonuses to Damage
    #[serde(rename = "b", alias = "Damage")]
    Damage,
    /// Bonus Sneak Attack Dice
    #[serde(rename = "i", alias = "Dice")]
    Dice,
}

impl SneakAttack {
    /// All possible values of [`SneakAttack`]
    pub const ALL: [Self; 3] = [Self::Attack, Self::Damage, Self::Dice];
}

impl Display for SneakAttack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Attack => write!(f, "Sneak Attack Hit"),
            Self::Damage => write!(f, "Sneak Attack Damage"),
            Self::Dice => write!(f, "Sneak Attack Dice"),
        }
    }
}

impl StaticOptions for SneakAttack {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}

impl ToAttribute for SneakAttack {
    fn to_attribute(self) -> Attribute {
        Attribute::SneakAttack(self)
    }
}
