//! Pulls together all the bonuses and calculates the bonuses for each attribute

mod calculation;
pub use calculation::*;

mod inserting;
pub use inserting::*;

mod buffer;

use std::collections::HashMap;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, get_base_bonuses},
};

/// TODO: Documentation
pub struct Breakdowns {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    cache: HashMap<Attribute, f32>,
    children: HashMap<BonusSource, Vec<Attribute>>,
}

impl Breakdowns {
    pub fn new() -> Self {
        let mut breakdowns = Breakdowns {
            bonuses: HashMap::new(),
            cache: HashMap::new(),
            children: HashMap::new(),
        };

        breakdowns.add_bonuses(get_base_bonuses());

        breakdowns
    }
}

impl Default for Breakdowns {
    fn default() -> Self {
        Self::new()
    }
}
