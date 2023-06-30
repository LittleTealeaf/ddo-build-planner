use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{types::Ability, Attribute, DefaultBonuses, TrackAttribute},
    bonus::{Bonus, BonusSource, BonusType, CloneBonus},
};

/// The different saving throws that a character can have bonuses to
///
/// The three main saving throws are [`Fortitude`], [`Reflex`], and [`Will`]. There is a [`SavingThrow::All`] entry that will clone bonuses to the three main bonuses. Additionally, there are subsidary bonuses like bonuses against [`Traps`]
///
/// [`Fortitude`]: SavingThrow::Fortitude
/// [`Reflex`]: SavingThrow::Reflex
/// [`Will`]: SavingThrow::Will
/// [`Traps`]: SavingThrow::Traps
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SavingThrow {
        /// Bonus to [`Fortitude`], [`Reflex`], and [`Will`] Saving Throws.
    ///
    /// [`Fortitude`]: SavingThrow::Fortitude
    /// [`Reflex`]: SavingThrow::Reflex
    /// [`Will`]: SavingThrow::Will
    All,
    /// Fortitude Saving Throw
    Fortitude,
    /// Bonus to Saving Throws against Poison
    Reflex,
    /// Bonus to Saving Throws agaisnt Traps
    Will,
    /// Bonus to Saving Throws against Enchantments
    Poison,
    /// Bonus to Saving Throws against Diseases
    Disease,
    /// Reflex Saving Throws
    Traps,
    /// Bonus to Saving Throws against Spells
    Spell,
    /// Bonus to Saving Throws against Magic
    Magic,
    /// Will Saving Throw
    Enchantment,
    /// Bonus to Saving Throws against Illusion
    Illusion,
    /// Bonus to Saving Throws against Fear
    Fear,
    /// Bonus to Saving Throws against Curses
    Curse,
}

impl SavingThrow {
    /// Lists the three main bonuses: [`Fortitude`], [`Reflex`], and [`Will`]
    ///
    /// [`Fortitude`]: SavingThrow::Fortitude
    /// [`Reflex`]: SavingThrow::Reflex
    /// [`Will`]: SavingThrow::Will
    pub const VALUES: [Self; 3] = [Self::Fortitude, Self::Reflex, Self::Will];
}

impl Display for SavingThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SavingThrow::Fortitude => write!(f, "Fortitude"),
            SavingThrow::Poison => write!(f, "Poison"),
            SavingThrow::Disease => write!(f, "Disease"),
            SavingThrow::Reflex => write!(f, "Reflex"),
            SavingThrow::Traps => write!(f, "Traps"),
            SavingThrow::Spell => write!(f, "Spell"),
            SavingThrow::Magic => write!(f, "Magic"),
            SavingThrow::Will => write!(f, "Will"),
            SavingThrow::Enchantment => write!(f, "Enchantment"),
            SavingThrow::Illusion => write!(f, "Illusion"),
            SavingThrow::Fear => write!(f, "Fear"),
            SavingThrow::Curse => write!(f, "Curse"),
            SavingThrow::All => write!(f, "All"),
        }
    }
}

impl CloneBonus for SavingThrow {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::VALUES
                .map(|st| {
                    Bonus::new(
                        st.into(),
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

impl DefaultBonuses for SavingThrow {
    fn get_default_bonuses() -> Vec<Bonus> {
        vec![
            Bonus::new(
                SavingThrow::Reflex.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Dexterity).into(),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                SavingThrow::Fortitude.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Constitution).into(),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                SavingThrow::Will.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Wisdom).into(),
                BonusSource::Base,
                None,
            ),
        ]
    }
}

impl TrackAttribute for SavingThrow {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(SavingThrow);

    #[test]
    fn all_is_not_tracked() {
        assert!(!SavingThrow::All.is_tracked());
        assert!(!Attribute::from(SavingThrow::All).is_tracked());
    }

    #[test]
    fn saving_throws_are_tracked() {
        let saving_throws = [
            SavingThrow::Fortitude,
            SavingThrow::Poison,
            SavingThrow::Disease,
            SavingThrow::Reflex,
            SavingThrow::Traps,
            SavingThrow::Spell,
            SavingThrow::Magic,
            SavingThrow::Will,
            SavingThrow::Enchantment,
            SavingThrow::Illusion,
            SavingThrow::Fear,
            SavingThrow::Curse,
        ];

        for st in saving_throws {
            assert!(st.is_tracked());
            assert!(Attribute::from(st).is_tracked());
        }
    }
}
