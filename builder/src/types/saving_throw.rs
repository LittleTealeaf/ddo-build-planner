use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
    /// The 3 main saving throws: [`Fortitude`], [`Reflex`], and [`Will`]
    ///
    /// [`Fortitude`]: SavingThrow::Fortitude
    /// [`Reflex`]: SavingThrow::Reflex
    /// [`Will`]: SavingThrow::Will
    pub const CORE_SAVING_THROWS: [SavingThrow; 3] = [Self::Fortitude, Self::Reflex, Self::Will];
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
