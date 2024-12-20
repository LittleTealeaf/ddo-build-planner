use core::iter::once;
use std::collections::HashSet;

use itertools::chain;
use rust_decimal::Decimal;
use utils::{hashmap::MapGetOrDefault, vecs::FilterRemove};

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource},
    types::flag::Flag,
};

use super::{buffer::Buffer, Breakdowns};

impl Breakdowns {
    /// Removes all bonuses with any of the provided [`BonusSources`].
    ///
    /// [`BonusSources`]: BonusSource
    pub fn remove_sources<I, B>(&mut self, sources: I)
    where
        I: IntoIterator<Item = B>,
        B: Into<BonusSource>,
    {
        let mut buffer = Buffer::new();

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

        let bonuses = bonuses.into_iter().inspect(|bonus| {
            sources.insert(bonus.source().clone());
        });

        let mut buffer = Buffer::new();
        buffer.insert_bonuses(bonuses);
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
    pub fn recalculate_attribute(&mut self, attribute: Attribute) {
        self.recalculate_attributes(once(attribute));
    }

    /// Forces the recalculation of several attributes
    pub fn recalculate_attributes<I>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut buffer = Buffer::new();
        buffer.insert_attributes(attributes);
        self.consume_buffer(buffer);
    }

    /// Forces the recalculation of all attributes
    pub fn recalculate_all_attributes(&mut self) {
        self.cache.condition.clear();
        self.cache.value.clear();

        let mut buffer = Buffer::new();
        buffer.insert_attributes(self.bonuses.keys().cloned());
        self.consume_buffer(buffer);
    }
}

impl Breakdowns {
    fn consume_buffer(&mut self, mut buffer: Buffer) {
        // List of attributes to recalculate
        let mut breakdowns = HashSet::new();

        while let Some((attribute, bonuses, forced)) = buffer.pop() {
            if self.cache.breakdowns.remove(&attribute).is_some() {
                breakdowns.insert(attribute.clone());
            }

            let initial_value = self
                .cache
                .attribute
                .remove(&attribute)
                .or_else(|| forced.then_some(Decimal::ZERO))
                .or_else(|| self.calculate_attribute(&attribute))
                .unwrap_or(Decimal::ZERO);

            self.bonuses.get_mut_or_default(&attribute).extend(bonuses);

            let current_value = self.calculate_attribute(&attribute).unwrap_or_default();

            if !forced && initial_value == current_value {
                continue;
            }

            self.update_caches(&attribute, &current_value);

            let source = BonusSource::Attribute(attribute.clone());

            buffer.insert_attributes(self.remove_bonuses_by_source(once(&source)));

            buffer.insert_attributes(
                self.get_dependants(&attribute)
                    .map(Bonus::attribute)
                    .cloned(),
            );

            let value = self.evaluate_attribute(&attribute);

            let attribute_bonuses = attribute.get_bonuses(value);

            let dynamic_bonuses = (value > Decimal::ZERO)
                .then(|| self.dynamic_bonuses.get(&attribute))
                .unwrap_or_default()
                .cloned();

            let mut children = HashSet::new();

            let bonuses = chain!(attribute_bonuses, dynamic_bonuses)
                .flatten()
                .map(|bonus| {
                    children.insert(bonus.attribute().clone());
                    bonus.to_bonus(source.clone())
                });

            buffer.insert_bonuses(bonuses);

            if !children.is_empty() {
                self.children.insert(source, children.into_iter().collect());
            }
        }

        for attribute in breakdowns {
            self.add_breakdown(attribute);
        }
    }

    fn get_dependants<'a>(&'a self, attribute: &'a Attribute) -> impl Iterator<Item = &'a Bonus> + 'a {
        let filter = |bonus: &&Bonus| bonus.has_attr_dependency(attribute);
        self.get_bonuses().filter(filter)
    }

    fn remove_bonuses_by_source<'a, I>(&'a mut self, sources: I) -> impl Iterator<Item = Bonus> + 'a
    where
        I: IntoIterator<Item = &'a BonusSource> + 'a,
    {
        let map = |source: &BonusSource| {
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
        };

        sources.into_iter().filter_map(map).flatten()
    }

    fn update_caches(&mut self, attribute: &Attribute, value: &Decimal) {
        fn filter_cache<K, V>(attribute: &Attribute) -> impl Fn(&K, &mut V) -> bool + '_
        where
            K: AttributeDependencies,
        {
            |key, _| !key.has_attr_dependency(attribute)
        }

        self.cache.value.retain(filter_cache(attribute));
        self.cache.condition.retain(filter_cache(attribute));

        match &attribute {
            Attribute::Flag(Flag::HasToggle(toggle)) => {
                if value > &Decimal::ZERO {
                    self.cache.toggles.insert(*toggle);
                } else {
                    self.cache.toggles.remove(toggle);
                }
            }
            Attribute::Flag(Flag::HasSlider(slider)) => {
                if value > &Decimal::ZERO {
                    self.cache.sliders.insert(*slider);
                } else {
                    self.cache.sliders.remove(slider);
                }
            }
            _ => {}
        }
    }
}
