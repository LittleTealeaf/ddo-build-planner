//! Summoned Attributes
use std::fmt::{self, Display};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::{Bonus, BonusTemplate, CloneBonus},
};

use super::ability::Ability;

/// Attributes pertaining to summoned creatures, charmed minions, pets, and hirelings
#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SummonedAttribute {
    /// Provides bonuses to ability scores for summoned creatures
    AbilityScore(Ability),
}

impl GetBonuses for SummonedAttribute {
    fn get_bonuses(&self, _: Decimal) -> Option<Vec<BonusTemplate>> {
        None
    }
}

impl CloneBonus for SummonedAttribute {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::AbilityScore(ability) => ability.clone_bonus(bonus),
        }
    }
}

impl Display for SummonedAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbilityScore(ability) => write!(f, "{ability} score"),
        }
    }
}

impl ToAttribute for SummonedAttribute {
    fn to_attribute(self) -> Attribute {
        Attribute::SummonedAttribute(self)
    }
}

impl StaticOptions for SummonedAttribute {
    fn get_static() -> impl Iterator<Item = Self> {
        Ability::get_static().map(Self::AbilityScore)
    }
}
