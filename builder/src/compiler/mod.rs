//! Handles the compilation and calculations of [`Bonuses`].
//!
//! [`Bonuses`]: crate::bonus::Bonus

mod buffer;
mod calculation;
mod inserting;

use std::collections::HashMap;

pub use calculation::*;
pub use inserting::*;

use crate::{
    attribute::Attribute,
    bonus::{get_base_bonuses, Bonus, BonusSource},
};

/// Compiles and calculates attribut values from a set of [`Bonus`] entries.
///
/// Internally, this uses [`OrdMap`] to efficiently store bonuses in a `HashMap` structure without the need of deriving [`Hash`].
///
/// This will handle any bonuses that different attributes may give (such as [`Attribute::Ability`] giving bonuses to [`Attribute::AbilityModifier`]), as well as cloned bonuses (such as [`Ability::All`] being split off into each of the abilities)
///
/// Note that the compiler must be mutable for most of it's publicly-facing functions
///
/// # Examples
///
/// ```
/// use builder::{
///     attribute::{
///         Attribute,
///     },
///     bonus::{Bonus, BonusSource, Condition, BonusType},
///     compiler::Compiler,
///     types::sheltering::Sheltering,
/// };
///
/// let mut compiler = Compiler::default();
///
/// compiler.add_bonus(Bonus::new(Attribute::Sheltering(Sheltering::Magical), BonusType::Stacking, 5f32.into(), BonusSource::Custom(0), None));
///
/// assert_eq!(5f32, compiler.get_attribute(&Attribute::Sheltering(Sheltering::Magical)));
/// ```
///
///
/// [`Bonus`]: crate::bonus::Bonus
/// [`Ability::All`]: crate::attribute::types::Ability::All
pub struct Compiler {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    cache: HashMap<Attribute, f32>,
    children: HashMap<BonusSource, Vec<Attribute>>,
}

impl Default for Compiler {
    fn default() -> Self {
        let mut new = Self {
            bonuses: HashMap::new(),
            cache: HashMap::new(),
            children: HashMap::new(),
        };

        new.add_bonuses(get_base_bonuses());

        new
    }
}
