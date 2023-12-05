//! Each of the possible saving throws
public_modules!(bonuses);

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::public_modules;

/// The different saving throws that a character can have bonuses to
///
/// The three main saving throws are [`Fortitude`], [`Reflex`], and [`Will`]. There is a [`SavingThrow::All`] entry that will clone bonuses to the three main bonuses. Additionally, there are subsidary bonuses like bonuses against [`Traps`]
///
/// [`Fortitude`]: SavingThrow::Fortitude
/// [`Reflex`]: SavingThrow::Reflex
/// [`Will`]: SavingThrow::Will
/// [`Traps`]: SavingThrow::Traps
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
    /// The 3 main saving throws: [`Fortitude`], [`Reflex`], and [`Will`]
    ///
    /// [`Fortitude`]: SavingThrow::Fortitude
    /// [`Reflex`]: SavingThrow::Reflex
    /// [`Will`]: SavingThrow::Will
    pub const PRIMARY: [Self; 3] = [Self::Fortitude, Self::Reflex, Self::Will];

    /// All secondary saving throws
    pub const SECONDARY: [Self; 9] = [
        Self::Poison,
        Self::Disease,
        Self::Traps,
        Self::Spell,
        Self::Magic,
        Self::Enchantment,
        Self::Illusion,
        Self::Fear,
        Self::Curse,
    ];

    /// Gets the parent saving throw.
    ///
    /// For example, The [`Illusion`] saving throw is a subsidary of [`Will`], thus [`Will`] is the
    /// parent saving throw.
    ///
    /// Saving Throws that do not have parents, such as primary saving throws ([`Fortitude`],
    /// [`Reflex`], or [`Will`]) or [`All`] will return None
    pub const fn get_parent(&self) -> Option<Self> {
        match self {
            Self::Poison | Self::Disease => Some(Self::Fortitude),
            Self::Traps | Self::Spell | Self::Magic => Some(Self::Reflex),
            Self::Enchantment | Self::Illusion | Self::Fear | Self::Curse => Some(Self::Will),
            _ => None,
        }
    }
}

impl Display for SavingThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fortitude => write!(f, "Fortitude"),
            Self::Poison => write!(f, "Poison"),
            Self::Disease => write!(f, "Disease"),
            Self::Reflex => write!(f, "Reflex"),
            Self::Traps => write!(f, "Traps"),
            Self::Spell => write!(f, "Spell"),
            Self::Magic => write!(f, "Magic"),
            Self::Will => write!(f, "Will"),
            Self::Enchantment => write!(f, "Enchantment"),
            Self::Illusion => write!(f, "Illusion"),
            Self::Fear => write!(f, "Fear"),
            Self::Curse => write!(f, "Curse"),
            Self::All => write!(f, "All"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_has_no_parent() {
        assert!(SavingThrow::Fortitude.get_parent().is_none());
        assert!(SavingThrow::Reflex.get_parent().is_none());
        assert!(SavingThrow::Will.get_parent().is_none());
    }

    #[test]
    fn all_has_no_parent() {
        assert!(SavingThrow::All.get_parent().is_none());
    }

    #[test]
    fn secondaries_have_correct_parent() {
        let list = [
            (SavingThrow::Poison, SavingThrow::Fortitude),
            (SavingThrow::Disease, SavingThrow::Fortitude),
            (SavingThrow::Traps, SavingThrow::Reflex),
            (SavingThrow::Spell, SavingThrow::Reflex),
            (SavingThrow::Magic, SavingThrow::Reflex),
            (SavingThrow::Enchantment, SavingThrow::Will),
            (SavingThrow::Illusion, SavingThrow::Will),
            (SavingThrow::Fear, SavingThrow::Will),
            (SavingThrow::Curse, SavingThrow::Will),
        ];

        for (secondary, primary) in list {
            assert_eq!(secondary.get_parent(), Some(primary));
        }
    }
}
