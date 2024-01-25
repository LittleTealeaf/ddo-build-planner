use std::collections::HashSet;

use itertools::chain;
use rust_decimal::Decimal;
use utils::hashmap::MapGetMutOrDefault;

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, ToValue},
};

use super::{buffer::Buffer, Breakdowns};

impl Breakdowns {
    /// Removes all bonuses with any of the provided [`BonusSources`]
    ///
    /// [`BonusSources`]: BonusSource
    pub fn remove_sources(&mut self, sources: impl IntoIterator<Item = impl Into<BonusSource>>) {
        let mut buffer = Buffer::empty();

        let sources = sources.into_iter().map(Into::into).collect::<Vec<_>>();

        buffer.insert_attributes(self.remove_bonuses_by_source(&sources));

        self.consume_buffer(buffer);
    }

    /// Removes all bonuses with the provided [`BonusSource`]
    pub fn remove_source(&mut self, source: impl Into<BonusSource>) {
        self.remove_sources([source]);
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
        self.recalculate_attributes([attribute]);
    }

    /// Forces the recalculation of several attributes
    pub fn recalculate_attributes(&mut self, attributes: impl IntoIterator<Item = Attribute>) {
        let mut buffer = Buffer::empty();
        buffer.insert_attributes(attributes);
        self.consume_buffer(buffer);
    }
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
                self.value_cache
                    .retain(|key, _| !key.has_attr_dependency(&attribute));
                self.condition_cache
                    .retain(|key, _| !key.has_attr_dependency(&attribute));

                let source = BonusSource::Attribute(attribute.clone());

                buffer.insert_attributes(self.remove_bonuses_by_source([&source]));

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

    fn remove_bonuses_by_source<'a>(
        &'a mut self,
        sources: impl IntoIterator<Item = &'a BonusSource> + 'a,
    ) -> impl Iterator<Item = Bonus> + 'a {
        sources
            .into_iter()
            .filter_map(|source| {
                let children = self.children.remove(source)?;

                let mut bonuses = Vec::new();

                for child in &children {
                    if let Some(set) = self.bonuses.get_mut(child) {
                        let items = set.iter().enumerate();

                        let indexes = items
                            .filter_map(|(index, item)| item.source().eq(source).then_some(index))
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
