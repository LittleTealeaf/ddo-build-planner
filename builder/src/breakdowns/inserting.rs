use std::collections::HashSet;

use utils::{float::ErrorMargin, hashmap::MapGetMutOrDefault};

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource, CloneBonus},
};

use super::{buffer::Buffer, Breakdowns};

impl Breakdowns {
    pub fn remove_sources(&mut self, sources: impl IntoIterator<Item = BonusSource>) {
        self.insert_bonuses(sources.into_iter().map(Bonus::dummy));
    }

    pub fn remove_source(&mut self, source: BonusSource) {
        self.remove_sources([source]);
    }

    pub fn insert_bonus(&mut self, bonus: Bonus) {
        self.insert_bonuses([bonus]);
    }

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

        buffer.insert_attributes(self.remove_source_bonuses(sources));

        self.consume_buffer(buffer);
    }

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
                .cache
                .remove(&attribute)
                .or_else(|| forced.then_some(0f32))
                .or_else(|| self.calculate_attribute(&attribute))
                .unwrap_or(0f32);

            self.bonuses.get_mut_or_default(&attribute).extend(bonuses);

            if forced || !initial_value.within_margin(&self.get_attribute(&attribute)) {
                let source = BonusSource::Attribute(attribute);

                buffer.insert_attributes(self.get_dependants(attribute).copied());
                buffer.insert_attributes(self.remove_source_bonuses([source]));

                let value = self.get_attribute(&attribute);

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

    fn remove_source_bonuses<'a>(
        &'a mut self,
        sources: impl IntoIterator<Item = BonusSource> + 'a,
    ) -> impl Iterator<Item = Attribute> + 'a {
        sources
            .into_iter()
            .filter_map(|source| {
                let children = self.children.remove(&source)?;

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
                            set.swap_remove(index);
                        }
                    }
                }

                Some(children)
            })
            .flatten()
    }

    fn get_dependants(&self, attribute: Attribute) -> impl Iterator<Item = &Attribute> + '_ {
        self.get_bonuses().filter_map(move |bonus| {
            bonus
                .has_attr_dependency(attribute)
                .then_some(bonus.get_attribute())
        })
    }
}
