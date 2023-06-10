use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, CloneBonus},
    utils::EnumBinaryMap,
};

use super::{attribute_queue::AttributeQueue, Compiler};

// Supporting Functions
impl Compiler {
    fn remove_source_from_children(&mut self, source: BonusSource, children: Vec<Attribute>) {
        for child in children {
            if let Some(set) = self.bonuses.get_mut(&child) {
                // Enumerate items in the set
                let items = set.iter().enumerate();

                // Filter map to the indexes of bonuses with the same source, and collect into a vec
                let indexes = items
                    .filter_map(|(index, item)| item.get_source().eq(&source).then_some(index))
                    .rev()
                    .collect_vec();

                // Swap-remove each index
                for index in indexes {
                    set.swap_remove(index);
                }
            }
        }
    }

    fn get_bonus_iter(&self) -> impl Iterator<Item = &Bonus> {
        self.bonuses
            .iter()
            .flat_map(|(_, bonus_set)| bonus_set.iter())
    }
}

// Public Functions
impl Compiler {
    /// Adds one bonus to the compiler set.
    pub fn add_bonus(&mut self, bonus: Bonus) {
        self.add_bonuses(vec![bonus]);
    }

    /// Adds a set of bonuses to the compiler.
    pub fn add_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
        // Add any cloned bonuses
        bonuses.append(
            &mut bonuses
                .iter()
                .filter_map(|bonus| bonus.get_attribute().clone_bonus(bonus))
                .flatten()
                .collect(),
        );

        // Create and populate queue of attributes to update
        let mut attribute_queue = AttributeQueue::default();
        attribute_queue.insert(bonuses.iter().map(Bonus::get_attribute).collect(), false);

        // Updating sources that are being inserted
        {
            // Collect sources and their child attributes
            let sources = EnumBinaryMap::from(
                bonuses
                    .iter()
                    .map(|bonus| (bonus.get_source(), bonus.get_attribute())),
            );

            // Iterate over each source, and if there was a prior mapping of source to children, remove those attributes and add them (forcefully) to the attribute queue
            sources.into_iter().for_each(|(source, set)| {
                if let Some(children) = self.children.insert(source, set) {
                    attribute_queue.insert(children.clone(), true);
                    self.remove_source_from_children(source, children);
                }
            });
        }

        // Initialize the update bonuses
        let mut update_bonuses = EnumBinaryMap::from(
            bonuses
                .into_iter()
                .map(|bonus| (bonus.get_attribute(), bonus)),
        );

        // Fetch the next attribute from the queue
        while let Some((attribute, force_update)) = attribute_queue.get_next_attribute() {
            let initial_value = {
                if let Some(value) = self.cache.remove(&attribute) {
                    // First try to REMOVE it from the cache
                    value
                } else if force_update {
                    // Otherwise, if it's a forced update, return 0f32
                    0f32
                } else {
                    // Since we're not forcing, we need to check the difference, so calcualte the value
                    self.calculate_attribute(&attribute).unwrap_or(0f32)
                }
            };

            // If there are any new bonuses, add those
            if let Some(mut bonuses) = update_bonuses.remove(&attribute) {
                self.bonuses
                    .get_mut_or_default(&attribute)
                    .append(&mut bonuses);
            }

            // Either check if it's forced, or if the initial value is different from the original
            if force_update || initial_value != self.get_attribute(&attribute) {
                // Add any attributes with this attribute as a dependant into the iterator
                {
                    // First get the attributes that have this attribute as a dependant
                    let attributes = self
                        .get_bonus_iter()
                        .filter_map(|bonus| {
                            bonus
                                .get_dependencies()?
                                .contains(&attribute)
                                .then_some(bonus.get_attribute())
                        })
                        .collect_vec();
                    // Add those to the attribute queue
                    attribute_queue.insert(attributes, true);
                }

                // Source
                let source = attribute.into();

                // Removes any bonuses that are children
                if let Some(children) = self.children.remove(&source) {
                    self.remove_source_from_children(source, children);
                }

                // Calculate the value
                let value = self.get_attribute(&attribute);

                if let Some(mut bonuses) = attribute.get_bonuses(value) {
                    // Add any cloned bonuses
                    bonuses.append(
                        &mut bonuses
                            .iter()
                            .filter_map(|bonus| bonus.get_attribute().clone_bonus(bonus))
                            .flatten()
                            .collect(),
                    );

                    let updated_attributes = EnumBinaryMap::from(
                        EnumBinaryMap::from(
                            bonuses
                                .into_iter()
                                .map(|bonus| (bonus.get_attribute(), bonus)),
                        )
                        .into_iter()
                        .map(|(attribute, mut set)| {
                            update_bonuses
                                .get_mut_or_default(&attribute)
                                .append(&mut set);
                            attribute
                        }),
                    )
                    .into_iter()
                    .map(|(key, _)| key)
                    .collect_vec();

                    self.children.insert(source, updated_attributes.clone());
                    attribute_queue.insert(updated_attributes, false);
                }
            }
        }
    }
}
