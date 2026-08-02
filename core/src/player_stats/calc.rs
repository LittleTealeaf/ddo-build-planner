mod attribute;
mod breakdown;

use std::collections::{BinaryHeap, HashMap, HashSet};

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, BonusCondition, BonusValue},
    player_stats::{calc::breakdown::AttributeBreakdown, Bonuses, PlayerStats},
};

#[derive(Debug, Clone, Default)]
pub(super) struct BreakdownCache {
    values: HashMap<BonusValue, Decimal>,
    conditions: HashMap<BonusCondition, bool>,
    breakdowns: HashMap<Attribute, AttributeBreakdown>,
}

pub(super) struct StatsCalculator<'a> {
    cache: &'a mut BreakdownCache,
    bonuses: &'a Bonuses,
}

impl StatsCalculator<'_> {
    pub fn reset_attributes<I>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut attributes = BinaryHeap::from_iter(attributes);
        let mut last_attribute = None;
        let mut breakdowns_to_update = HashSet::new();

        while let Some(attribute) = attributes.pop() {
            if Some(&attribute) == last_attribute.as_ref() {
                continue;
            }

            self.cache
                .values
                .retain(|key, _| !key.contains_attribute(&attribute));
            self.cache
                .values
                .retain(|key, _| !key.contains_attribute(&attribute));

            attributes.extend(self.bonuses.get_dependant_attributes(&attribute).cloned());

            if self.cache.breakdowns.contains_key(&attribute) {
                breakdowns_to_update.insert(attribute.clone());
            }

            last_attribute = Some(attribute);
        }

        for attribute in breakdowns_to_update {
            self.update_breakdown(attribute);
        }
    }
}

impl PlayerStats {
    #[must_use]
    pub(super) const fn calculator(&mut self) -> StatsCalculator<'_> {
        StatsCalculator {
            cache: &mut self.calc_cache,
            bonuses: &self.bonuses,
        }
    }

    pub fn evaluate_attribute(&mut self, attribute: Attribute, snapshot: Option<u32>) -> Decimal {
        self.calculator().evaluate_attribute(attribute, snapshot)
    }

    pub fn track_breakdown(&mut self, attribute: Attribute) {
        if !self.calc_cache.breakdowns.contains_key(&attribute) {
            self.calculator().update_breakdown(attribute);
        }
    }

    pub fn remove_tracked_breakdown(&mut self, attribute: &Attribute) -> Option<AttributeBreakdown> {
        self.calc_cache.breakdowns.remove(attribute)
    }

    pub fn clear_tracked_breakdowns(&mut self) {
        self.calc_cache.breakdowns.clear();
    }

    pub fn get_tracked_breakdown(&mut self, attribute: &Attribute) -> Option<&AttributeBreakdown> {
        self.calc_cache.breakdowns.get(attribute)
    }
}
