use core::iter::once;
use std::collections::HashSet;

use itertools::{chain, Itertools};
use rust_decimal::Decimal;
use utils::{hashmap::MapGetMutOrDefault, vecs::FilterRemove};

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, ToValue},
};

use super::{buffer::Buffer, Breakdowns};

pub struct BreakdownBatch<'a> {
    breakdowns: &'a mut Breakdowns,
    sources: HashSet<BonusSource>,
    bonuses: Vec<Bonus>,
    attributes: Vec<Attribute>,
}

impl Breakdowns {
    pub fn edit(&mut self) -> BreakdownBatch<'_> {
        BreakdownBatch {
            breakdowns: self,
            sources: HashSet::new(),
            bonuses: Vec::new(),
            attributes: Vec::new(),
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

impl<'a> BreakdownBatch<'a> {
    pub fn apply(self) {
        let mut buffer = Buffer::new();

        let sources = chain!(self.sources.iter(), self.bonuses.iter().map(Bonus::source));

        let removed_bonuses = self.breakdowns.remove_by_source(sources);

        buffer.insert_attributes(removed_bonuses);
        buffer.insert_attributes(self.attributes);
        buffer.insert_bonuses(self.bonuses);

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

    pub fn remove_sources<I, B>(mut self, sources: I) -> Self
    where
        I: IntoIterator<Item = B>,
        B: Into<BonusSource>,
    {
        self.sources.extend(sources.into_iter().map(Into::into));
        self
    }

    pub fn remove_source<S>(self, source: S) -> Self
    where
        S: Into<BonusSource>,
    {
        self.remove_sources(once(source))
    }

    pub fn insert_bonuses<I, B>(mut self, bonuses: I) -> Self
    where
        I: IntoIterator<Item = B>,
        B: Into<Bonus>,
    {
        self.bonuses.extend(bonuses.into_iter().map(Into::into));
        self
    }

    pub fn insert_bonus<B>(self, bonus: B) -> Self
    where
        B: Into<Bonus>,
    {
        self.insert_bonuses(once(bonus))
    }

    pub fn recalculate_attributes<I, A>(mut self, attributes: I) -> Self
    where
        I: IntoIterator<Item = A>,
        A: Into<Attribute>,
    {
        self.attributes
            .extend(attributes.into_iter().map(Into::into));
        self
    }

    pub fn recalculate_attribute<A>(self, attribute: A) -> Self
    where
        A: Into<Attribute>,
    {
        self.recalculate_attributes(once(attribute))
    }

    pub fn recalculate_all_attributes(self) -> Self {
        self.breakdowns.value_cache.clear();
        self.breakdowns.condition_cache.clear();
        let keys = self.breakdowns.bonuses.keys().cloned().collect_vec();
        self.recalculate_attributes(keys)
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
