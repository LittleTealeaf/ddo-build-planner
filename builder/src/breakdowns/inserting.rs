use std::collections::HashSet;

use itertools::chain;
use rust_decimal::Decimal;
use utils::hashmap::MapGetMutOrDefault;

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, CloneBonus},
};

use super::{buffer::Buffer, Breakdowns};

impl Breakdowns {
    /// Removes all bonuses with any of the provided [`BonusSources`]
    ///
    /// [`BonusSources`]: BonusSource
    pub fn remove_sources(&mut self, sources: impl IntoIterator<Item = impl Into<BonusSource>>) {
        self.insert_bonuses(sources.into_iter().map(Bonus::dummy));
    }

    /// Removes all bonuses with the provided [`BonusSource`]
    pub fn remove_source(&mut self, source: impl Into<BonusSource>) {
        self.insert_bonuses([Bonus::dummy(source)]);
    }

    /// Inserts a single bonus into the breakdowns. This also removes all bonuses that have the
    /// same bonus source.
    pub fn insert_bonus(&mut self, bonus: Bonus) {
        self.insert_bonuses([bonus]);
    }

    /// Inserts several bonuses into the breakdowns. This also removes all bonuses that have the
    /// same bonus source.
    pub fn insert_bonuses(&mut self, bonuses: impl IntoIterator<Item = Bonus>) {
        let mut sources = HashSet::new();

        let bonuses = bonuses
            .into_iter()
            .flat_map(|bonus| {
                sources.insert(*bonus.get_source());
                [
                    bonus
                        .get_attribute()
                        .clone_bonus(&bonus)
                        .unwrap_or_default(),
                    vec![bonus],
                ]
            })
            .flatten();

        let mut buffer = Buffer::create(bonuses);

        let updated_bonuses = self.remove_bonuses_by_source(sources).collect::<Vec<_>>();

        let updated_attributes = updated_bonuses.into_iter().map(|bonus| {
            self.bonus_cache.remove(&bonus);
            *bonus.get_attribute()
        });

        buffer.insert_attributes(updated_attributes);

        self.consume_buffer(buffer);
    }
}

impl Breakdowns {
    fn consume_buffer(&mut self, mut buffer: Buffer) {
        for bonus in buffer.get_bonuses() {
            let attribute = bonus.get_attribute();
            let source = bonus.get_source();

            let set = self.children.get_mut_or_default(source);
            if !set.contains(attribute) {
                set.push(*attribute);
            }
        }

        while let Some((attribute, bonuses, forced)) = buffer.pop() {
            let initial_value = self
                .attribute_cache
                .remove(&attribute)
                .or_else(|| forced.then_some(Decimal::ZERO))
                .or_else(|| self.calculate_attribute(attribute))
                .unwrap_or(Decimal::ZERO);

            self.bonuses.get_mut_or_default(&attribute).extend(bonuses);

            if forced || initial_value != self.get_attr(&attribute) {
                let source = BonusSource::Attribute(attribute);

                let updated_bonuses = chain!(
                    self.remove_bonuses_by_source([source]).collect::<Vec<_>>(),
                    self.get_dependants(attribute).cloned().collect::<Vec<_>>(),
                );

                let updated_attributes = updated_bonuses.map(|bonus| {
                    self.bonus_cache.remove(&bonus);
                    *bonus.get_attribute()
                });

                buffer.insert_attributes(updated_attributes);

                let value = self.get_attr(&attribute);

                if let Some(bonuses) = attribute.get_bonuses(value) {
                    self.children.insert(
                        source,
                        bonuses.iter().map(Bonus::get_attribute).copied().collect(),
                    );

                    buffer.insert_bonuses(bonuses);
                }
            }
        }
    }

    fn get_dependants(&self, attribute: Attribute) -> impl Iterator<Item = &Bonus> + '_ {
        self.get_bonuses()
            .filter(move |bonus| bonus.has_attr_dependency(attribute))
    }

    fn remove_bonuses_by_source<'a>(
        &'a mut self,
        sources: impl IntoIterator<Item = BonusSource> + 'a,
    ) -> impl Iterator<Item = Bonus> + 'a {
        sources
            .into_iter()
            .filter_map(|source| {
                let children = self.children.remove(&source)?;

                let mut bonuses = Vec::new();

                for child in &children {
                    if let Some(set) = self.bonuses.get_mut(child) {
                        let items = set.iter().enumerate();

                        let indexes = items
                            .filter_map(|(index, item)| {
                                item.get_source().eq(&source).then_some(index)
                            })
                            .rev()
                            .collect::<Vec<_>>();

                        for index in indexes {
                            bonuses.push(set.swap_remove(index));
                        }
                    }
                }
                Some(bonuses)
            })
            .flatten()
    }
}
