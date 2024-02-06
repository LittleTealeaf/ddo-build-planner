use core::iter::once;
use std::collections::HashSet;

use itertools::chain;
use rust_decimal::Decimal;
use utils::{hashmap::MapGetMutOrDefault, vecs::FilterRemove};

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, ToValue},
};

use super::{buffer::Buffer, Breakdowns};

impl Breakdowns {
    /// Removes all bonuses with any of the provided [`BonusSources`]
    ///
    /// [`BonusSources`]: BonusSource
    pub fn remove_sources<I, B>(&mut self, sources: I)
    where
        I: IntoIterator<Item = B>,
        B: Into<BonusSource>,
    {
        let mut buffer = Buffer::empty();

        let sources = sources.into_iter().map(Into::into).collect::<Vec<_>>();

        buffer.insert_attributes(self.remove_bonuses_by_source(&sources));

        self.consume_buffer(buffer);
    }

    /// Removes all bonuses with the provided [`BonusSource`]
    pub fn remove_source<S>(&mut self, source: S)
    where
        S: Into<BonusSource>,
    {
        self.remove_sources(once(source));
    }

    /// Inserts a single bonus into the breakdowns. This also removes all bonuses that have the
    /// same bonus source.
    pub fn insert_bonus(&mut self, bonus: Bonus) {
        self.insert_bonuses(once(bonus));
    }

    /// Inserts several bonuses into the breakdowns. This also removes all bonuses that have the
    /// same bonus source.
    pub fn insert_bonuses<I>(&mut self, bonuses: I)
    where
        I: IntoIterator<Item = Bonus>,
    {
        let mut sources = HashSet::new();

        let bonuses = bonuses.into_iter().map(|bonus| {
            sources.insert(bonus.source().clone());
            bonus
        });

        let mut buffer = Buffer::create(bonuses);

        buffer.insert_attributes(self.remove_bonuses_by_source(&sources));

        for bonus in buffer.get_bonuses() {
            let attribute = bonus.attribute();
            let source = bonus.source();

            let set = self.children.get_mut_or_default(source);
            if !set.contains(attribute) {
                set.push(attribute.clone());
            }
        }

        self.consume_buffer(buffer);
    }

    /// Forces the recalculation of an attribute
    pub fn recalcualte_attribute(&mut self, attribute: Attribute) {
        self.recalculate_attributes(once(attribute));
    }

    /// Forces the recalculation of several attributes
    pub fn recalculate_attributes<I>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut buffer = Buffer::empty();
        buffer.insert_attributes(attributes);
        self.consume_buffer(buffer);
    }

    /// Forces the recalculation of all attributes
    pub fn recalculate_all_attributes(&mut self) {
        self.value_cache.clear();
        self.condition_cache.clear();
        let mut buffer = Buffer::empty();
        buffer.insert_attributes(self.bonuses.keys().cloned());
        self.consume_buffer(buffer);
    }
}

fn filter_attr_deps<K, V>(attribute: &Attribute) -> impl Fn(&K, &mut V) -> bool + '_
where
    K: AttributeDependencies,
{
    |key, _| key.has_attr_dependency(attribute)
}

impl Breakdowns {
    fn consume_buffer(&mut self, mut buffer: Buffer) {
        while let Some((attribute, bonuses, forced)) = buffer.pop() {
            let initial_value = self
                .value_cache
                .remove(&attribute.clone().to_value())
                .or_else(|| forced.then_some(Decimal::ZERO))
                .or_else(|| self.calculate_attribute(&attribute))
                .unwrap_or(Decimal::ZERO);

            self.bonuses.get_mut_or_default(&attribute).extend(bonuses);

            if forced || initial_value != self.get_attribute(attribute.clone()) {
                self.value_cache.retain(filter_attr_deps(&attribute));
                self.condition_cache.retain(filter_attr_deps(&attribute));

                let source = BonusSource::Attribute(attribute.clone());

                buffer.insert_attributes(self.remove_bonuses_by_source(once(&source)));

                buffer.insert_attributes(
                    self.get_dependants(&attribute)
                        .map(Bonus::attribute)
                        .cloned(),
                );

                let value = self.get_attribute(attribute.clone());

                let attribute_bonuses = attribute.get_bonuses(value);

                let dynamic_bonuses = (value > Decimal::ZERO)
                    .then(|| self.dynamic_bonuses.get(&attribute))
                    .unwrap_or_default();

                let bonuses = chain!(&attribute_bonuses, dynamic_bonuses)
                    .flatten()
                    .collect::<Vec<_>>();

                if !bonuses.is_empty() {
                    self.children.insert(
                        source.clone(),
                        bonuses
                            .iter()
                            .map(|bonus| bonus.attribute())
                            .cloned()
                            .collect(),
                    );
                    buffer.insert_bonuses(
                        bonuses
                            .into_iter()
                            .cloned()
                            .map(|bonus| bonus.to_bonus(source.clone())),
                    );
                }
            }
        }
    }

    fn get_dependants<'a>(&'a self, attribute: &'a Attribute) -> impl Iterator<Item = &Bonus> + '_ {
        self.get_bonuses()
            .filter(|bonus| bonus.has_attr_dependency(attribute))
    }

    fn remove_bonuses_by_source<'a, I>(&'a mut self, sources: I) -> impl Iterator<Item = Bonus> + 'a
    where
        I: IntoIterator<Item = &'a BonusSource> + 'a,
    {
        sources
            .into_iter()
            .filter_map(|source| {
                let children = self.children.remove(source)?;

                let mut bonuses = Vec::new();

                for child in &children {
                    if let Some(set) = self.bonuses.get_mut(child) {
                        bonuses.extend(set.remove_filter(|item| item.source().eq(source)));
                    }
                }
                Some(bonuses)
            })
            .flatten()
    }
}
