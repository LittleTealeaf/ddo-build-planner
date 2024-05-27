//! Summoned Attributes
use core::fmt::{self, Display};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::{chain_tree, enums::StaticOptions};

use crate::{
    attribute::{Attribute, GetBonuses, ToAttribute},
    bonus::{Bonus, BonusTemplate, CloneBonus},
};

use super::{ability::Ability, sheltering::Sheltering};

/// Attributes pertaining to summoned creatures, charmed minions, pets, and hirelings
#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SummonedAttribute {
    /// Provides bonuses to ability scores for summoned creatures
    AbilityScore(Ability),
    /// Sheltering
    Sheltering(Sheltering),
}

impl GetBonuses for SummonedAttribute {
    fn get_bonuses(&self, _: Decimal) -> Option<Vec<BonusTemplate>> {
        None
    }
}
impl CloneBonus for SummonedAttribute {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::AbilityScore(ability) => matches!(ability, Ability::All).then(|| {
                Ability::ABILITIES
                    .map(|ability| {
                        bonus.clone_with_attribute(Attribute::SummonedAttribute(
                            Self::AbilityScore(ability),
                        ))
                    })
                    .to_vec()
            }),
            Self::Sheltering(shelter) => matches!(shelter, Sheltering::Both).then(|| {
                [Sheltering::Physical, Sheltering::Magical]
                    .map(|sheltering| bonus.clone_with_attribute(Self::Sheltering(sheltering)))
                    .to_vec()
            }),
        }
    }
}

impl Display for SummonedAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbilityScore(ability) => write!(f, "{ability} score"),
            Self::Sheltering(shelter) => write!(f, "{shelter}"),
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
        chain_tree!(
            Ability::get_static().map(Self::AbilityScore),
            Sheltering::get_static().map(Self::Sheltering)
        )
    }
}
