#![allow(missing_docs)]

use core::iter::{empty, once, Empty};
use std::collections::HashSet;

use itertools::{chain, Itertools};
use rust_decimal::Decimal;
use utils::{hashmap::MapGetMutOrDefault, vecs::FilterRemove};

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, ToValue},
};

use super::{buffer::Buffer, Breakdowns};

impl Breakdowns {
    pub fn edit(
        &mut self,
    ) -> BreakdownBatch<'_, Empty<BonusSource>, Empty<Bonus>, Empty<Attribute>> {
        BreakdownBatch {
            breakdowns: self,
            sources: empty(),
            bonuses: empty(),
            attributes: empty(),
        }
    }

    pub fn insert_bonus<B>(&mut self, bonus: B)
    where
        B: Into<Bonus>,
    {
        self.edit().insert_bonus(bonus).apply();
    }

    pub fn insert_bonuses<I, B>(&mut self, bonuses: I)
    where
        B: Into<Bonus>,
        I: IntoIterator<Item = B>,
    {
        self.edit().insert_bonuses(bonuses).apply();
    }

    pub fn recalculate_attribute<A>(&mut self, attribute: A)
    where
        A: Into<Attribute>,
    {
        self.edit().recalculate_attribute(attribute).apply();
    }

    pub fn recalculate_attributes<I, A>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.edit().recalculate_attributes(attributes).apply();
    }

    pub fn recalculate_all_attributes(&mut self) {
        self.edit().recalculate_all_attributes().apply();
    }

    pub fn remove_source<S>(&mut self, source: S)
    where
        S: Into<BonusSource>,
    {
        self.edit().remove_source(source).apply();
    }

    pub fn remove_sources<I, S>(&mut self, sources: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<BonusSource>,
    {
        self.edit().remove_sources(sources).apply();
    }
}

pub struct BreakdownBatch<'a, S, B, A>
where
    S: Iterator<Item = BonusSource>,
    B: Iterator<Item = Bonus>,
    A: Iterator<Item = Attribute>,
{
    breakdowns: &'a mut Breakdowns,
    sources: S,
    bonuses: B,
    attributes: A,
}

impl<'a, S1, B1, A1> BreakdownBatch<'a, S1, B1, A1>
where
    S1: Iterator<Item = BonusSource>,
    B1: Iterator<Item = Bonus>,
    A1: Iterator<Item = Attribute>,
{
    pub fn apply(self) {
        let mut buffer = Buffer::new();

        let mut sources = self.sources.collect::<HashSet<_>>();

        let bonuses = self.bonuses.map(|bonus| {
            sources.insert(bonus.source().clone());
            bonus
        });

        buffer.insert_bonuses(bonuses);

        let removed_bonuses = self.breakdowns.remove_by_source(&sources);

        buffer.insert_attributes(chain!(removed_bonuses.map(Into::into), self.attributes));

        for bonus in buffer.get_bonuses() {
            let attribute = bonus.attribute();
            let source = bonus.source();

            let set = self.breakdowns.children.get_mut_or_default(source);
            if !set.contains(attribute) {
                set.push(attribute.clone());
            }
        }

        self.breakdowns.consume_buffer(buffer);
    }

    #[must_use]
    pub fn remove_sources<I, S>(
        self,
        sources: I,
    ) -> BreakdownBatch<'a, impl Iterator<Item = BonusSource>, B1, A1>
    where
        S: Into<BonusSource>,
        I: IntoIterator<Item = S>,
    {
        BreakdownBatch {
            sources: chain!(self.sources, sources.into_iter().map(Into::into)),
            bonuses: self.bonuses,
            attributes: self.attributes,
            breakdowns: self.breakdowns,
        }
    }

    #[must_use]
    pub fn remove_source<S>(
        self,
        source: S,
    ) -> BreakdownBatch<'a, impl Iterator<Item = BonusSource>, B1, A1>
    where
        S: Into<BonusSource>,
    {
        self.remove_sources(once(source))
    }

    #[must_use]
    pub fn insert_bonuses<I, B>(
        self,
        bonuses: I,
    ) -> BreakdownBatch<'a, S1, impl Iterator<Item = Bonus>, A1>
    where
        I: IntoIterator<Item = B>,
        B: Into<Bonus>,
    {
        BreakdownBatch {
            sources: self.sources,
            bonuses: chain!(self.bonuses, bonuses.into_iter().map(Into::into)),
            attributes: self.attributes,
            breakdowns: self.breakdowns,
        }
    }

    #[must_use]
    pub fn insert_bonus<B>(
        self,
        bonus: B,
    ) -> BreakdownBatch<'a, S1, impl Iterator<Item = Bonus>, A1>
    where
        B: Into<Bonus>,
    {
        self.insert_bonuses(once(bonus))
    }

    #[must_use]
    pub fn recalculate_attributes<I, A>(
        self,
        attributes: I,
    ) -> BreakdownBatch<'a, S1, B1, impl Iterator<Item = Attribute>>
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        BreakdownBatch {
            sources: self.sources,
            bonuses: self.bonuses,
            attributes: chain!(self.attributes, attributes.into_iter().map(Into::into)),
            breakdowns: self.breakdowns,
        }
    }

    #[must_use]
    pub fn recalculate_attribute<A>(
        self,
        attribute: A,
    ) -> BreakdownBatch<'a, S1, B1, impl Iterator<Item = Attribute>>
    where
        A: Into<Attribute>,
    {
        self.recalculate_attributes(once(attribute))
    }

    #[must_use]
    pub fn recalculate_all_attributes(
        self,
    ) -> BreakdownBatch<'a, S1, B1, impl Iterator<Item = Attribute>> {
        self.breakdowns.value_cache.clear();
        self.breakdowns.condition_cache.clear();
        let attributes = self.breakdowns.bonuses.keys().cloned().collect_vec();
        self.recalculate_attributes(attributes)
    }
}

impl Breakdowns {
    fn remove_by_source<'a, I>(&'a mut self, sources: I) -> impl Iterator<Item = Bonus> + 'a
    where
        I: IntoIterator<Item = &'a BonusSource> + 'a,
    {
        sources
            .into_iter()
            .filter_map(|source| {
                let children = self.children.remove(source)?;

                let mut bonuses = Vec::new();

                for child in &children {
                    let Some(set) = self.bonuses.get_mut(child) else {
                        continue;
                    };

                    let filter = |item: &Bonus| item.source().eq(source);
                    let items = set.filter_remove(filter);
                    bonuses.extend(items);
                }
                Some(bonuses)
            })
            .flatten()
    }

    fn consume_buffer(&mut self, mut buffer: Buffer) {
        fn filter_cache<K, V>(attribute: &Attribute) -> impl Fn(&K, &mut V) -> bool + '_
        where
            K: AttributeDependencies,
        {
            |key, _| key.has_attr_dependency(attribute)
        }

        while let Some((attribute, bonuses, forced)) = buffer.pop() {
            let initial_value = self
                .value_cache
                .remove(&attribute.clone().to_value())
                .or_else(|| forced.then_some(Decimal::ZERO))
                .or_else(|| self.calculate_attribute(&attribute))
                .unwrap_or(Decimal::ZERO);

            self.bonuses.get_mut_or_default(&attribute).extend(bonuses);

            if !forced && initial_value == self.get_attribute(attribute.clone()) {
                continue;
            }

            self.value_cache.retain(filter_cache(&attribute));
            self.condition_cache.retain(filter_cache(&attribute));

            let source = BonusSource::Attribute(attribute.clone());

            buffer.insert_attributes(self.remove_by_source(once(&source)));

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

            if bonuses.is_empty() {
                continue;
            }

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

    fn get_dependants<'a>(&'a self, attribute: &'a Attribute) -> impl Iterator<Item = &Bonus> + '_ {
        let filter = |bonus: &&Bonus| bonus.has_attr_dependency(attribute);
        self.get_bonuses().filter(filter)
    }
}
