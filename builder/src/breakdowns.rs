//! Pulls together all the bonuses and calculates the bonuses for each attribute

mod base;
mod breakdown;
mod buffer;
mod dynamic;
mod evaluation;
mod inserting;

use core::fmt::{self, Display};
use std::collections::HashMap;

use im::OrdSet;
use rust_decimal::Decimal;

pub use breakdown::*;
pub use dynamic::DynamicBonus;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusTemplate, Condition, HasDice, Value},
    types::{slider::Slider, toggle::Toggle},
};

use self::base::get_base_bonuses;

/// Breakdowns is an object that handles calculating the final attribute values for a character.
///
/// This object is used to both display final attribute values ([`Self::get_attribute`]),
/// as well as list out the bonus breakdown of on particular attribute ([`Self::get_breakdowns`])
/// of a particular variable
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Breakdowns {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    #[serde(flatten)]
    cache: BreakdownCache,
    children: HashMap<BonusSource, Vec<Attribute>>,
    dynamic_bonuses: HashMap<Attribute, Vec<BonusTemplate>>,
    dice_strategy: DiceStrategy,
}

/// Determines the strategy used when evaluating dice in bonuses
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiceStrategy {
    /// Dice will always roll 1s, or the lowest value
    Minimum,
    /// Dice will evaluate to the average roll
    Average,
    /// Dice will always roll the highest value possible
    Maximum,
}

impl DiceStrategy {
    /// All of the possible dice strategies
    pub const VALUES: [Self; 3] = [Self::Minimum, Self::Average, Self::Maximum];
}

impl Display for DiceStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minimum => write!(f, "Minimum"),
            Self::Average => write!(f, "Average"),
            Self::Maximum => write!(f, "Maximum"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct BreakdownCache {
    value: HashMap<Value, Decimal>,
    condition: HashMap<Condition, bool>,
    attribute: HashMap<Attribute, Decimal>,
    breakdowns: HashMap<Attribute, AttributeBreakdown>,
    toggles: OrdSet<Toggle>,
    sliders: OrdSet<Slider>,
}

/// Simple methods for creating new instances, and obtaining a list of bonuses or attributes
/// calculated.
impl Breakdowns {
    /// Creates a new [`Breakdowns`] instance, ready for use.
    /// This will also populate the instance with all the default bonuses, which implement
    /// the logic used for all characters.
    ///
    /// # Notes
    /// There are additional methods appended to this object using traits within the `data` crate.
    /// These methods may add additional 'dynamic' bonuses, other other bonuses generated from
    /// serialized data.
    #[must_use]
    pub fn new() -> Self {
        let mut breakdowns = Self {
            bonuses: HashMap::new(),
            cache: BreakdownCache::default(),
            children: HashMap::new(),
            dynamic_bonuses: HashMap::new(),
            dice_strategy: DiceStrategy::Average,
        };

        breakdowns.insert_bonuses(get_base_bonuses());

        breakdowns
    }

    /// Returns an iterator of all of the bonuses currently in the breakdowns
    pub fn get_bonuses(&self) -> impl Iterator<Item = &Bonus> {
        self.bonuses.values().flatten()
    }

    /// Returns an iterator of attributes and their values
    pub fn iter_attributes(&mut self) -> impl Iterator<Item = (Attribute, Decimal)> + '_ {
        let attributes = self.bonuses.keys().cloned().collect::<Vec<_>>();

        attributes
            .into_iter()
            .map(|attribute| (attribute.clone(), self.evaluate_value(&attribute.into())))
    }

    /// Returns all toggles that should be displayed
    #[must_use]
    pub const fn get_displayed_toggles(&self) -> &OrdSet<Toggle> {
        &self.cache.toggles
    }

    /// Returns the list of toggles that are turned on
    pub fn get_active_toggles(&mut self) -> impl Iterator<Item = Toggle> + '_ {
        let toggles = self.get_displayed_toggles().clone();

        toggles
            .into_iter()
            .filter(|toggle| self.evaluate_attribute(&Attribute::Toggle(*toggle)) > Decimal::ZERO)
    }

    /// Returns all sliders that should be displayed
    #[must_use]
    pub const fn get_displayed_sliders(&self) -> &OrdSet<Slider> {
        &self.cache.sliders
    }

    /// Returns a list of sliders and their current values
    pub fn get_active_sliders(&mut self) -> impl Iterator<Item = (Slider, Decimal)> + '_ {
        let sliders = self.get_displayed_sliders().clone();

        sliders
            .into_iter()
            .map(|slider| (slider, self.evaluate_attribute_from(slider)))
    }

    /// Returns the current dice strategy being used
    #[must_use]
    pub const fn dice_strategy(&self) -> DiceStrategy {
        self.dice_strategy
    }

    /// Sets the dice strategy, and recalculates any attributes that depend on any dice value
    pub fn set_dice_strategy(&mut self, strategy: DiceStrategy) {
        self.dice_strategy = strategy;

        self.cache.value.retain(|val, _| !val.has_dice());
        self.cache.condition.retain(|val, _| !val.has_dice());

        let attributes = self
            .get_bonuses()
            .filter(|&bonus| bonus.has_dice())
            .map(Bonus::attribute)
            .cloned()
            .collect::<Vec<_>>();

        self.recalculate_attributes(attributes);
    }
}

impl Default for Breakdowns {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_base_bonuses_have_base_source() {
        for bonus in get_base_bonuses() {
            assert_eq!(
                bonus.source(),
                &BonusSource::Base,
                "Does not have base bonus: {bonus:?}"
            );
        }
    }
}
