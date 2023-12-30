//! Pulls together all the bonuses and calculates the bonuses for each attribute

mod calculation;
mod inserting;

use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{get_base_bonuses, Bonus, BonusSource},
};

/// Calculates the final attribute values for the character.
pub struct Breakdowns {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    cache: HashMap<Attribute, Decimal>,
    children: HashMap<BonusSource, Vec<Attribute>>,
}

impl Breakdowns {
    #[must_use]
    /// Creates a new Breakdowns instance
    pub fn new() -> Self {
        let mut breakdowns = Self {
            bonuses: HashMap::new(),
            cache: HashMap::new(),
            children: HashMap::new(),
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
        let attributes = self.bonuses.keys().copied().collect::<Vec<_>>();

        attributes
            .into_iter()
            .map(|attribute| (attribute, self.get_attribute(&attribute)))
    }
}

impl Default for Breakdowns {
    fn default() -> Self {
        Self::new()
    }
}
