//! Summoned Attributes
use std::fmt::Display;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute, TrackAttribute},
    bonus::{Bonus, CloneBonus},
};

use super::ability::Ability;

/// Attributes pertaining to summoned creatures, charmed minions, pets, and hirelings
#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SummonedAttribute {
    /// Provides bonuses to ability scores for summoned creatures
    AbilityScore(Ability),
}

impl GetBonuses for SummonedAttribute {
    fn get_bonuses(&self, _: Decimal) -> Option<Vec<Bonus>> {
        None
    }
}

impl CloneBonus for SummonedAttribute {
    fn clone_bonus(&self, bonus: &crate::bonus::Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::AbilityScore(ability) => ability.clone_bonus(bonus),
        }
    }
}

impl TrackAttribute for SummonedAttribute {
    fn is_tracked(&self) -> bool {
        match self {
            Self::AbilityScore(ability) => ability.is_tracked(),
        }
    }
}

impl Display for SummonedAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AbilityScore(ability) => write!(f, "{ability} score"),
        }
    }
}

impl ToAttribute for SummonedAttribute {
    fn to_attribute(self) -> crate::attribute::Attribute {
        Attribute::SummonedAttribute(self)
    }
}
