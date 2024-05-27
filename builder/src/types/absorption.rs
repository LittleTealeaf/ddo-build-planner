//! Describes different absorption types, since absorption isn't additive

use core::fmt;
use core::iter::once;

use fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

use super::damage_type::DamageType;

/// Describes both the total absorption and individual bonuses to absorption
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Absorption {
    /// Final Absorption
    #[serde(rename = "t", alias = "Total")]
    Total(DamageType),
    /// Bonus to each damage type
    #[serde(rename = "b", alias = "Bonus")]
    Bonus(DamageType, AbsorptionSource),
}

impl Display for Absorption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Total(dam) => write!(f, "{dam} Absorption"),
            Self::Bonus(dam, bonus_type) => write!(f, "{dam} Absorption Bonus: {bonus_type}"),
        }
    }
}

impl StaticOptions for Absorption {
    fn get_static() -> impl Iterator<Item = Self> {
        DamageType::get_static().flat_map(|damage_type| {
            chain!(
                once(Self::Total(damage_type)),
                AbsorptionSource::get_static()
                    .map(move |bonus_type| { Self::Bonus(damage_type, bonus_type) })
            )
        })
    }
}

impl ToAttribute for Absorption {
    fn to_attribute(self) -> Attribute {
        Attribute::Absorption(self)
    }
}

/// The absorption source / stacking source for bonuses
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AbsorptionSource {
    /// Bonuses from an item
    #[serde(rename = "e", alias = "Enhancement")]
    Enhancement,
    /// Insightful
    #[serde(rename = "i", alias = "Insightful")]
    Insightful,
    /// Artifact Bonuses
    #[serde(rename = "a", alias = "Artifact")]
    Artifact,
    /// Energy Sheathe from Draconic Incarnation
    #[serde(rename = "s", alias = "EnergySheathe")]
    EnergySheathe,

    /// Arcane Past Lives
    #[serde(rename = "p", alias = "ArcanePastLife")]
    ArcanePastLife,
    /// Guild Ship Buff
    #[serde(rename = "g", alias = "Guild")]
    Guild,
}

impl Display for AbsorptionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enhancement => write!(f, "Enhancement"),
            Self::Insightful => write!(f, "Insightful"),
            Self::Artifact => write!(f, "Artifact"),
            Self::EnergySheathe => write!(f, "Energy Sheathe"),
            Self::ArcanePastLife => write!(f, "Arcane Past Life"),
            Self::Guild => write!(f, "Guild Buffs"),
        }
    }
}

impl StaticOptions for AbsorptionSource {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Self::Enhancement,
            Self::EnergySheathe,
            Self::Insightful,
            Self::ArcanePastLife,
            Self::Guild,
            Self::Artifact,
        ]
        .into_iter()
    }
}
