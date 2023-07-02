use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, DefaultBonuses, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusSource, BonusType, CloneBonus},
};

/// The different abilities that a character has
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Ability {
    /// All values
    All,
    /// Represents how strong the character is
    #[serde(rename = "Str")]
    Strength,
    /// Represents how flexible the character is
    #[serde(rename = "Dex")]
    Dexterity,
    /// Determines the character's health
    #[serde(rename = "Con")]
    Constitution,
    /// Represents how smart the character is
    #[serde(rename = "Int")]
    Intelligence,
    /// Represents how wise the character is.
    #[serde(rename = "Wis")]
    Wisdom,
    /// Represents how charismatic the character is.
    #[serde(rename = "Cha")]
    Charisma,
}

impl Ability {
    /// Represents the 6 different values that [`Ability`] can be (without [`Ability::All`])
    pub const VALUES: [Ability; 6] = [
        Ability::Strength,
        Ability::Dexterity,
        Ability::Constitution,
        Ability::Intelligence,
        Ability::Wisdom,
        Ability::Charisma,
    ];

    fn modifier_bonus<T>(&self, attribute: T, value: f32) -> Bonus
    where
        Attribute: From<T>,
    {
        Bonus::new(
            attribute.into(),
            BonusType::AbilityModifier,
            value.into(),
            Attribute::AbilityModifier(*self).into(),
            None,
        )
    }
}

impl GetBonuses for Ability {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (!matches!(self, Self::All)).then(|| {
            vec![Bonus::new(
                Attribute::AbilityModifier(*self),
                BonusType::Stacking,
                ((value - 10f32) / 2f32).floor().into(),
                Attribute::Ability(*self).into(),
                None,
            )]
        })
    }
}

impl DefaultBonuses for Ability {
    fn get_default_bonuses() -> Vec<Bonus> {
        Self::VALUES
            .map(|ability| {
                Bonus::new(
                    Attribute::Ability(ability),
                    BonusType::Stacking,
                    8f32.into(),
                    BonusSource::Base,
                    None,
                )
            })
            .to_vec()
    }
}

impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::VALUES
                .map(|ability| {
                    Bonus::new(
                        ability.into(),
                        bonus.get_type(),
                        bonus.get_value(),
                        bonus.get_source(),
                        bonus.get_condition(),
                    )
                })
                .to_vec()
        })
    }
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ability::Strength => write!(f, "Strength"),
            Ability::Dexterity => write!(f, "Dexterity"),
            Ability::Constitution => write!(f, "Constitution"),
            Ability::Intelligence => write!(f, "Intelligence"),
            Ability::Wisdom => write!(f, "Wisdom"),
            Ability::Charisma => write!(f, "Charisma"),
            Ability::All => write!(f, "All"),
        }
    }
}

impl TrackAttribute for Ability {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(Ability);

    #[test]
    fn all_is_not_tracked() {
        assert!(!Ability::All.is_tracked());
        assert!(!Attribute::Ability(Ability::All).is_tracked());
        assert!(!Attribute::AbilityModifier(Ability::All).is_tracked());
    }

    #[test]
    fn abilities_are_tracked() {
        for ability in Ability::VALUES {
            assert!(ability.is_tracked());
            assert!(Attribute::Ability(ability).is_tracked());
            assert!(Attribute::AbilityModifier(ability).is_tracked());
        }
    }
}
