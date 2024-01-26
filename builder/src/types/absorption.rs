//! Describes different absorption types, since absorption isn't additive

use std::fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{attribute::{Attribute, ToAttribute}, bonus::BonusType};

use super::damage_type::DamageType;

/// Describes both the total absorption and indivdual bonuses to absorption
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Absorption {
    /// Final Absorption
    Total(DamageType),
    /// Bonus to each damage type
    Bonus(DamageType, BonusType),
}

impl Display for Absorption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Total(dam) => write!(f, "{dam} Absorption"),
            Self::Bonus(dam, bonus_type) => write!(f, "{bonus_type} bonus to {dam} Absorption"),
        }
    }
}

impl StaticOptions for Absorption {
    fn get_static() -> impl Iterator<Item = Self> {
        DamageType::get_static().flat_map(|damage_type| {
            chain!(
                [Self::Total(damage_type)],
                BonusType::get_static()
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
