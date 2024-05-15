//! Pulls together all the bonuses and calculates the bonuses for each attribute

mod base;
mod breakdown;
mod buffer;
mod dynamic;
mod evaluation;
mod inserting;

use core::ops::{AddAssign, SubAssign};
use std::collections::HashMap;

use rust_decimal::Decimal;

pub use breakdown::*;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusTemplate, Condition, HasDice, Value},
};

use self::base::get_base_bonuses;

/// Breakdowns is an object that handles calculating the final attribute values for a character.
/// This object is used to both display final attribute values ([`Self::get_attribute`]),
/// as well as list out the bonus breakdown of on particular attribute ([`Self::get_breakdowns`])
/// of a particular variable
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Breakdowns {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    value_cache: HashMap<Value, Decimal>,
    condition_cache: HashMap<Condition, bool>,
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
            value_cache: HashMap::new(),
            condition_cache: HashMap::new(),
            children: HashMap::new(),
            dynamic_bonuses: HashMap::new(),
            dice_strategy: DiceStrategy::Average,
        };

        breakdowns += get_base_bonuses();
        breakdowns.children.remove(&BonusSource::Base);

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

    /// Returns the current dice strategy being used
    #[must_use]
    pub const fn dice_strategy(&self) -> DiceStrategy {
        self.dice_strategy
    }

    /// Sets the dice strategy, and recalculates any attributes that depend on any dice value
    pub fn set_dice_strategy(&mut self, strategy: DiceStrategy) {
        self.dice_strategy = strategy;

        self.value_cache.retain(|val, _| !val.has_dice());
        self.condition_cache.retain(|val, _| !val.has_dice());

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

impl AddAssign<Bonus> for Breakdowns {
    fn add_assign(&mut self, rhs: Bonus) {
        self.insert_bonus(rhs);
    }
}

impl<I> AddAssign<I> for Breakdowns
where
    I: IntoIterator<Item = Bonus>,
{
    fn add_assign(&mut self, rhs: I) {
        self.insert_bonuses(rhs);
    }
}

impl SubAssign<BonusSource> for Breakdowns {
    fn sub_assign(&mut self, rhs: BonusSource) {
        self.remove_source(rhs);
    }
}

impl<I> SubAssign<I> for Breakdowns
where
    I: IntoIterator<Item = BonusSource>,
{
    fn sub_assign(&mut self, rhs: I) {
        self.remove_sources(rhs);
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
