//! Pulls together all the bonuses and calculates the bonuses for each attribute

mod breakdown;
mod buffer;
mod calculation;
mod dynamic;
mod inserting;

use std::collections::HashMap;

use rust_decimal::Decimal;

pub use breakdown::*;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{get_base_bonuses, Bonus, BonusSource, BonusTemplate, Condition, Value},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct EvalBonus {
    value: Decimal,
    condition: bool,
}

/// Calculates the final attribute values for the character.
#[derive(Debug, Serialize, Deserialize)]
pub struct Breakdowns {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    value_cache: HashMap<Value, Decimal>,
    condition_cache: HashMap<Condition, bool>,
    children: HashMap<BonusSource, Vec<Attribute>>,
    dynamic_bonuses: HashMap<Attribute, Vec<BonusTemplate>>,
}

impl Breakdowns {
    #[must_use]
    /// Creates a new Breakdowns instance
    pub fn new() -> Self {
        let mut breakdowns = Self {
            bonuses: HashMap::new(),
            value_cache: HashMap::new(),
            condition_cache: HashMap::new(),
            children: HashMap::new(),
            dynamic_bonuses: HashMap::new(),
        };

        breakdowns.insert_bonuses(get_base_bonuses());
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
            .map(|attribute| (attribute.clone(), self.get_attribute(attribute)))
    }
}

impl Default for Breakdowns {
    fn default() -> Self {
        Self::new()
    }
}
