//! Describes different absorption types, since absorption isn't additive

use std::fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::attribute::{Attribute, ToAttribute};

use super::damage_type::DamageType;

/// Describes both the total absorption and indivdual bonuses to absorption
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Absorption {
    /// Final Absorption
    Total(DamageType),
    /// Bonus to each damage type
    Bonus(DamageType, AbsorptionSource),
}

impl Display for Absorption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
                [Self::Total(damage_type)],
                AbsorptionSource::get_static()
                    .map(move |bonus_type| { Self::Bonus(damage_type, bonus_type) })
            )
        })
    }
}

impl ToAttribute for Absorption {
    fn to_attribute(self) -> crate::attribute::Attribute {
        Attribute::Absorption(self)
    }
}

/// The absorption soruce / stacking source for bonuses
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AbsorptionSource {
    /// Bonuses from an item
    Item,
    /// Energy Sheathe from Draconic Incarnation
    EnergySheathe,

    /// Arcane Past Lives
    ArcanePastLife,
    /// Guild Ship Buff
    Guild,

}

impl Display for AbsorptionSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Item => write!(f, "Item"),
            Self::EnergySheathe => write!(f, "Energy Sheathe"),
            Self::ArcanePastLife => write!(f, "Arcane Past Life"),
            Self::Guild => write!(f, "Guild Buffs"),
        }
    }
}

impl StaticOptions for AbsorptionSource {
    fn get_static() -> impl Iterator<Item = Self> {
        [Self::Item, Self::EnergySheathe, Self::ArcanePastLife, Self::Guild].into_iter()
    }
}
