mod attribute;
mod breakdown;

use std::collections::{BinaryHeap, HashMap, HashSet};

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, BonusCondition, BonusValue},
    breakdown::{BonusStore, Breakdown},
};

#[derive(Debug, Clone, Default)]
pub(super) struct BreakdownCache {
    values: HashMap<BonusValue, Decimal>,
    conditions: HashMap<BonusCondition, bool>,
    breakdowns: HashMap<Attribute, ()>,
}

pub(super) struct BreakdownCalculator<'a> {
    cache: &'a mut BreakdownCache,
    bonuses: &'a BonusStore,
}

impl BreakdownCalculator<'_> {
    pub fn reset_attributes<I>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut attributes = BinaryHeap::from_iter(attributes);
        let mut last = None;
        let mut breakdowns_to_make = HashSet::new();

        while let Some(attribute) = attributes.pop() {
            if Some(&attribute) == last.as_ref() {
                continue;
            }

            self.cache
                .values
                .retain(|key, _| !key.contains_attribute(&attribute));
            self.cache
                .values
                .retain(|key, _| !key.contains_attribute(&attribute));

            attributes.extend(self.bonuses.get_dependant_attributes(&attribute).cloned());

            if self.cache.breakdowns.remove(&attribute).is_some() {
                breakdowns_to_make.insert(attribute.clone());
            }

            last = Some(attribute);
        }

        for attribute in breakdowns_to_make {
            todo!()
        }
    }
}

impl Breakdown {
    #[must_use]
    pub(super) const fn calculator(&mut self) -> BreakdownCalculator<'_> {
        BreakdownCalculator {
            cache: &mut self.calc_cache,
            bonuses: &self.bonuses,
        }
    }
}
