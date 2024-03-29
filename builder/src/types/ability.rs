//! Ability types
use core::fmt;

use fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

/// The different abilities that a character has
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Ability {
    /// All values
    #[serde(rename = "a", alias = "All")]
    All,
    /// Represents how strong the character is
    #[serde(rename = "str", alias = "Strength")]
    Strength,
    /// Represents how flexible the character is
    #[serde(rename = "dex", alias = "Dexterity")]
    Dexterity,
    /// Determines the character's health
    #[serde(rename = "con", alias = "Constitution")]
    Constitution,
    /// Represents how smart the character is
    #[serde(rename = "int", alias = "Intelligence")]
    Intelligence,
    /// Represents how wise the character is.
    #[serde(rename = "wis", alias = "Wisdom")]
    Wisdom,
    /// Represents how charismatic the character is.
    #[serde(rename = "cha", alias = "Charisma")]
    Charisma,
}

impl Ability {
    /// All abilities in the game.
    ///
    /// Does not include [`All`]
    ///
    /// [`All`]: Ability::All
    pub const ABILITIES: [Self; 6] = [
        Self::Strength,
        Self::Dexterity,
        Self::Constitution,
        Self::Intelligence,
        Self::Wisdom,
        Self::Charisma,
    ];
}

impl Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Strength => write!(f, "Strength"),
            Self::Dexterity => write!(f, "Dexterity"),
            Self::Constitution => write!(f, "Constitution"),
            Self::Intelligence => write!(f, "Intelligence"),
            Self::Wisdom => write!(f, "Wisdom"),
            Self::Charisma => write!(f, "Charisma"),
            Self::All => write!(f, "All Ability"),
        }
    }
}

impl ToAttribute for Ability {
    fn to_attribute(self) -> Attribute {
        Attribute::Ability(self)
    }
}

impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::ABILITIES
                .map(|ability| bonus.clone_into_attribute(ability))
                .to_vec()
        })
    }
}

impl StaticOptions for Ability {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!([Self::All], Self::ABILITIES)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        attribute::Attribute,
        bonus::{BonusSource, BonusType},
    };

    use super::*;

    #[test]
    fn clone_bonus_return_none_for_ability() {
        for ability in Ability::ABILITIES {
            let bonus = ability.clone_bonus(&Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Stacking,
                1,
                BonusSource::Debug(0),
                None,
            ));
            assert!(bonus.is_none());
        }
    }

    #[test]
    fn clone_bonus_returns_all_bonuses() {
        let bonus = Bonus::new(
            Ability::All,
            BonusType::Stacking,
            1,
            BonusSource::Debug(0),
            None,
        );

        let bonuses = Ability::All
            .clone_bonus(&bonus)
            .expect("Expected clone_bonus to return Some(_)");

        let attributes = bonuses
            .into_iter()
            .map(|bonus| bonus.attribute().clone())
            .collect::<Vec<_>>();

        for ability in Ability::ABILITIES {
            assert!(attributes.contains(&Attribute::Ability(ability)));
        }
    }
}
