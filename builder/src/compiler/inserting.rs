use itertools::Itertools;
use utils::ord::{IntoOrdGroupMap, IntoOrdSet};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, CloneBonus},
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

/// Adding or Inserting bonuses into the Compiler
impl Compiler {
    /// Adds a single bonus into the compiler.
    ///
    /// See [`Compiler::add_bonuses`] for implementation details.
    ///
    /// [`Compiler::add_bonuses`]: crate::compiler::Compiler::add_bonuses
    pub fn add_bonus(&mut self, bonus: Bonus) {
        self.add_bonuses(vec![bonus]);
    }

    /// Inserts a vector of [`Bonus`] entries into the compiler.
    ///
    /// Removes pre-existing bonuses with the same source, and updates all dependant [`Bonus`]
    /// entries and [`Attribute`] caches.
    ///
    /// # Examples
    ///
    /// ```
    /// use builder::{
    ///     compiler::Compiler,
    ///     attribute::Attribute,
    ///     bonus::{
    ///         Bonus,
    ///         BonusSource,
    ///         BonusType
    ///     }
    /// };
    ///
    /// let mut compiler = Compiler::default();
    ///
    /// let bonus_a = Bonus::new(
    ///     Attribute::SpellResistance,
    ///     BonusType::Enhancement,
    ///     10f32.into(),
    ///     BonusSource::Custom(0),
    ///     None
    /// );
    ///
    /// compiler.add_bonuses(vec![bonus_a]);
    ///
    /// assert!(compiler.get_attribute(&Attribute::SpellResistance) == 10f32);
    ///
    /// let bonus_b = Bonus::new(
    ///     Attribute::SpellResistance,
    ///     BonusType::Enhancement,
    ///     5f32.into(),
    ///     BonusSource::Custom(0),
    ///     None
    /// );
    ///
    /// // We can use add_bous to only add one bonus as well!
    /// compiler.add_bonus(bonus_b);
    ///
    /// assert!(compiler.get_attribute(&Attribute::SpellResistance) == 5f32);
    /// ```
    /// Any [`Bonus`] already in the [`Compiler`] with the same [`BonusSource`] as any [`Bonus`]
    /// being added will be removed.
    ///
    /// In the example above, We first added bonus to the SpellResistance equal to `10f32`. We
    /// verified that the compiler calculated that bonus to equal `10f32`. Next, we added a bonus
    /// *from the same source* to SpellResistance equal to `5f32`. Because the two bonuses share a
    /// [`BonusSource`], adding the second [`Bonus`] will remove the first [`Bonus`].
    ///
    ///
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
            let sources = bonuses
                .iter()
                .map(|bonus| (bonus.get_source(), bonus.get_attribute()))
                .into_grouped_ord_map();

            // Iterate over each source, and if there was a prior mapping of source to children, remove those attributes and add them (forcefully) to the attribute queue
            sources.into_iter().for_each(|(source, set)| {
                if let Some(children) = self.children.insert(source, set) {
                    attribute_queue.insert(children.clone(), true);
                    self.remove_source_from_children(source, children);
                }
            });
        }

        // Initialize the update bonuses
        let mut update_bonuses = bonuses
            .into_iter()
            .map(|bonus| (bonus.get_attribute(), bonus))
            .into_grouped_ord_map();

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
                if let Some(list) = self.bonuses.get_mut(&attribute) {
                    list.append(&mut bonuses);
                } else {
                    self.bonuses.insert(attribute, bonuses);
                }
                // self.bonuses
                //     .get_mut_or_default(&attribute)
                //     .append(&mut bonuses);
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

                    let bonuses_by_attribute = bonuses
                        .into_iter()
                        .map(|bonus| (bonus.get_attribute(), bonus))
                        .into_grouped_ord_map();

                    let updated_attributes = bonuses_by_attribute
                        .into_iter()
                        .map(|(attribute, mut set)| {
                            if let Some(value) = update_bonuses.get_mut(&attribute) {
                                value.append(&mut set);
                            } else {
                                update_bonuses.insert(attribute, set);
                            }
                            attribute
                        })
                        .into_ord_set()
                        .into_iter()
                        .collect_vec();

                    self.children.insert(source, updated_attributes.clone());
                    attribute_queue.insert(updated_attributes, false);
                }
            }
        }
    }
}
