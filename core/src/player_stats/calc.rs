mod attribute;
mod breakdown;

use std::collections::{BinaryHeap, HashMap};

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{traits::ContainsAttribute, BonusCondition, BonusValue},
    player_stats::{calc::breakdown::AttributeBreakdown, Bonuses, PlayerStats},
};

#[derive(Debug, Clone, Default)]
pub(super) struct BreakdownCache {
    values: HashMap<(BonusValue, Option<u32>), Decimal>,
    conditions: HashMap<(BonusCondition, Option<u32>), bool>,
    breakdowns: HashMap<Attribute, AttributeBreakdown>,
}

#[derive(derive_more::From)]
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
        let mut breakdowns_to_update = Vec::new();

        while let Some(attribute) = attributes.pop() {
            if Some(&attribute) == last_attribute.as_ref() {
                continue;
            }
            log::debug!("Clearing Attribute Cache: {attribute}");

            attributes.extend(self.bonuses.get_dependant_attributes(&attribute).cloned());

            self.cache
                .values
                .retain(|(key, _), _| !key.contains_attribute(&attribute));
            self.cache
                .conditions
                .retain(|(key, _), _| !key.contains_attribute(&attribute));

            if let Some(breakdown) = self.cache.breakdowns.remove(&attribute) {
                breakdowns_to_update.push(breakdown.into());
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

    pub fn evaluate_attribute<A>(&mut self, attribute: A, snapshot: Option<u32>) -> Decimal
    where
        A: Into<Attribute>,
    {
        self.calculator()
            .evaluate_attribute(attribute.into(), snapshot)
    }

    pub fn track_breakdown<A>(&mut self, attribute: Attribute) {
        if !self.calc_cache.breakdowns.contains_key(&attribute) {
            self.calculator().update_breakdown(attribute);
        }
    }

    pub fn remove_tracked_breakdown(
        &mut self,
        attribute: &Attribute,
    ) -> Option<AttributeBreakdown> {
        self.calc_cache.breakdowns.remove(attribute)
    }

    pub fn clear_tracked_breakdowns(&mut self) {
        self.calc_cache.breakdowns.clear();
    }

    pub fn get_tracked_breakdown(&mut self, attribute: &Attribute) -> Option<&AttributeBreakdown> {
        self.calc_cache.breakdowns.get(attribute)
    }
}
