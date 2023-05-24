use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

/// Describes any immunities that the player has
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Enum, Serialize, Deserialize)]
pub enum Immunity {
    /// Immunity to Magicis Missiles
    MagicMissile,
    /// Immunity to being negative leveled
    EnergyDrain,
    /// Immunity to instant death effects
    Death,
    /// Immunity to mummy rot
    MummyRot,
    /// Immunity to natural diseases
    NaturalDisease,
    /// Immunity to slippery surfaces
    SlipperySurfaces,
    /// Immunity to knockdown (most forms)
    Knockdown,
    /// Immunity to quell
    Quell,
    /// Immunity to petrification
    Petrification,
    /// Immunity to slow (most forms)
    Slow,
}

impl ToString for Immunity {
    fn to_string(&self) -> String {
        match self {
            Immunity::MagicMissile => String::from("Magic Missile"),
            Immunity::EnergyDrain => String::from("Energy Drain"),
            Immunity::MummyRot => String::from("Mummy Rot"),
            Immunity::NaturalDisease => String::from("Natural Diseases"),
            Immunity::SlipperySurfaces => String::from("Slippery Surfaces"),
            Immunity::Knockdown => String::from("Knockdown"),
            Immunity::Quell => String::from("Quell"),
            Immunity::Petrification => String::from("Petrification"),
            Immunity::Slow => String::from("Most Slow Forms"),
            Immunity::Death => String::from("Instant Death"),
        }
    }
}

impl From<Immunity> for Attribute {
    fn from(value: Immunity) -> Self {
        Attribute::Flag(value.into())
    }
}
