use itertools::Itertools;
use utils::{float::ErrorMargin, hashmap::MapGetMutOrDefault};

use crate::{
    attribute::{Attribute, AttributeDependencies},
    bonus::{Bonus, BonusSource},
};

use super::{buffer::Buffer, Compiler};

/// Functions to remove and add bonuses
impl Compiler {
    /// Removes all bonuses from the given soruces from the compiler
    pub fn remove_sources(&mut self, sources: impl IntoIterator<Item = BonusSource>) {
        self.add_bonuses(sources.into_iter().map(Bonus::dummy));
    }

    /// Removes all bonuses from a given source from the compiler
    pub fn remove_source(&mut self, source: BonusSource) {
        self.remove_sources([source]);
    }

    /// Adds a single bonus to the compiler
    pub fn add_bonus(&mut self, bonus: Bonus) {
        self.add_bonuses([bonus]);
    }

    /// Adds multiple bonuses to the compiler
    pub fn add_bonuses(&mut self, bonuses: impl IntoIterator<Item = Bonus>) {
        let mut buffer = Buffer::default();
        buffer.insert_bonuses(bonuses, true);

        for source in buffer.get_sources() {
            self.remove_by_source(source);
        }

        self.consume_buffer(buffer);
    }
}

// Helper functions for adding bonuses
impl Compiler {
    fn consume_buffer(&mut self, mut buffer: Buffer) {
        while let Some((attribute, bonuses, forced)) = buffer.pop() {
            let initial_value = self
                .cache
                .remove(&attribute)
                .or_else(|| forced.then_some(0f32))
                .or_else(|| self.calculate_attribute(&attribute))
                .unwrap_or(0f32);

            self.insert_bonuses(attribute, bonuses);

            if forced || !initial_value.within_margin(&self.get_attribute(&attribute)) {
                // Add all dependants to the buffer
                buffer.insert_attributes(self.get_dependants(attribute));

                let source: BonusSource = attribute.into();

                self.remove_by_source(source);

                let value = self.get_attribute(&attribute);

                if let Some(bonuses) = attribute.get_bonuses(value) {
                    self.children
                        .insert(source, bonuses.iter().map(Bonus::get_attribute).collect());

                    buffer.insert_bonuses(bonuses, false);
                }
            }
        }
    }

    fn remove_by_source(&mut self, source: BonusSource) {
        if let Some(children) = self.children.remove(&source) {
            for child in children {
                if let Some(set) = self.bonuses.get_mut(&child) {
                    let items = set.iter().enumerate();

                    let indexes = items
                        .filter_map(|(index, item)| item.get_source().eq(&source).then_some(index))
                        .rev()
                        .collect_vec();

                    for index in indexes {
                        set.swap_remove(index);
                    }
                }
            }
        }
    }

    fn insert_bonuses(&mut self, attribute: Attribute, bonuses: impl IntoIterator<Item = Bonus>) {
        self.bonuses.get_mut_or_default(&attribute).extend(bonuses);
    }

    fn get_bonus_iter(&self) -> impl Iterator<Item = &Bonus> {
        self.bonuses.values().flatten()
    }

    fn get_dependants(&self, attribute: Attribute) -> impl Iterator<Item = Attribute> + '_ {
        self.get_bonus_iter().filter_map(move |bonus| {
            bonus
                .has_attr_dependency(attribute)
                .then_some(bonus.get_attribute())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{attribute::Attribute, compiler::Compiler, types::ability::Ability};

    #[test]
    fn get_dependants_for_default_bonuses() {
        // This assumes that there is a default bonus that links the Dexterity Score to the Dexterity Modifier

        let compiler = Compiler::default();

        assert!(compiler
            .get_dependants(Attribute::Ability(Ability::Dexterity))
            .next()
            .is_some());
    }
}
