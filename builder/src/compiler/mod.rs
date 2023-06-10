//! Handles the compilation and calculations of [`Bonuses`].
//!
//! [`Bonuses`]: crate::bonus::Bonus

mod attribute_queue;
mod calculation;
mod inserting;

pub use calculation::*;
pub use inserting::*;

use crate::{
    attribute::{Attribute, DefaultBonuses},
    bonus::{Bonus, BonusSource},
    utils::EnumBinaryMap,
};

/// Compiles and calculates attribut values from a set of [`Bonus`] entries.
///
/// Internally, this uses [`EnumBinaryMaps`] to efficiently store bonuses in a HashMap structure without the need of deriving [`Hash`].
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
///         types::{
///             Sheltering
///         }
///     },
///     bonus::{Bonus, BonusSource, Condition, BonusType},
///     compiler::Compiler
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
/// [`EnumBinaryMaps`]: crate::utils::EnumBinaryMap
/// [`Bonus`]: crate::bonus::Bonus
/// [`Ability::All`]: crate::attribute::types::Ability::All
pub struct Compiler {
    bonuses: EnumBinaryMap<Attribute, Vec<Bonus>>,
    cache: EnumBinaryMap<Attribute, f32>,
    children: EnumBinaryMap<BonusSource, Vec<Attribute>>,
}

impl Default for Compiler {
    fn default() -> Self {
        let mut new = Self {
            bonuses: EnumBinaryMap::default(),
            cache: EnumBinaryMap::default(),
            children: EnumBinaryMap::default(),
        };

        new.add_bonuses(Attribute::get_default_bonuses());

        new
    }
}
