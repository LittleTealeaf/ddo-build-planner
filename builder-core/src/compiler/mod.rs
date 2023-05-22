use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, Condition},
};

use self::{attribute_queue::AttributeQueue, bonus_set::BonusSet, clone_bonuses::clone_bonuses};

mod attribute_queue;
mod bonus_set;
mod clone_bonuses;
mod partial_bonus;

/// Compiles multiple bonuses and calculates resulting attribute values based on bonus rules.
///
/// The struct handles the core calculations of attribute bonuses.
pub struct AttributeCompiler {
    bonuses: BonusSet,
    children: HashMap<BonusSource, Vec<Attribute>>,
    cache: HashMap<Attribute, f32>,
}

impl Default for AttributeCompiler {
    #[inline]
    fn default() -> Self {
        Self {
            bonuses: BonusSet::new(),
            children: HashMap::new(),
            cache: HashMap::new(),
        }
    }
}

impl AttributeCompiler {
    /// Creates a new [AttributeCompiler] with no bonuses.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    fn calculate_attribute(&self, attribute: &Attribute) -> Option<f32> {
        Some(
            self.bonuses
                .get(attribute)?
                .iter()
                .filter(|bonus| match &bonus.conditions {
                    None => true,
                    Some(conditions) => conditions.iter().all(|condition| match condition {
                        Condition::Has(attr) => {
                            self.calculate_attribute(attr).unwrap_or(0f32) != 0f32
                        }
                        Condition::NotHave(attr) => {
                            self.calculate_attribute(attr).unwrap_or(0f32) == 0f32
                        }
                        Condition::Eq(attr, value) => {
                            self.calculate_attribute(attr).unwrap_or(0f32) == *value
                        }
                        Condition::Max(attr, value) => {
                            self.calculate_attribute(attr).unwrap_or(0f32) <= *value
                        }
                        Condition::Min(attr, value) => {
                            self.calculate_attribute(attr).unwrap_or(0f32) >= *value
                        }
                        Condition::NotEq(attr, value) => {
                            self.calculate_attribute(attr).unwrap_or(0f32) != *value
                        }
                    }),
                })
                .map(|bonus| (bonus.bonus_type, bonus.value))
                .into_group_map()
                .into_iter()
                .map(|(bonus_type, mut items)| {
                    let mut value = items.pop().unwrap();
                    if bonus_type.is_stacking() {
                        for item in items {
                            value += item;
                        }
                    } else {
                        for item in items {
                            if value < item {
                                value = item;
                            }
                        }
                    }
                    value
                })
                .sum(),
        )
    }

    /// Gets the calculated value of an attribute
    pub fn get_attribute(&mut self, attribute: &Attribute) -> f32 {
        if let Some(value) = self.cache.get(attribute) {
            return *value;
        }

        let value = self.calculate_attribute(attribute).unwrap_or(0f32);

        self.cache.insert(*attribute, value);

        value
    }

    /// Inserts a single attribute into the compiler.
    #[inline(always)]
    pub fn insert_bonus(&mut self, bonus: Bonus) {
        self.insert_bonuses(vec![bonus]);
    }

    /// Inserts a list of attributes into the compiler.
    pub fn insert_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
        // Adds additional attribute clones
        clone_bonuses(&mut bonuses);

        // Creates and initially sets queue
        let mut attribute_queue = AttributeQueue::new();
        attribute_queue.insert_attriubtes(
            bonuses.iter().map(Bonus::get_attribute).unique().collect(),
            false,
        );

        bonuses
            .iter()
            .map(|bonus| (bonus.get_source(), bonus.get_attribute()))
            .into_group_map()
            .into_iter()
            .for_each(|(source, set): (BonusSource, Vec<Attribute>)| {
                // Inserts new set
                if let Some(children) = self.children.insert(source, set) {
                    // If there was an old set, remove the sources from that
                    self.bonuses.remove_source_from(source, children);
                }
            });

        // Initializes list of bonuses to insert
        let mut update_bonuses = bonuses
            .into_iter()
            .map(|bonus| (bonus.get_attribute(), bonus))
            .into_group_map();

        // While the next attribute in the queue is inserted
        while let Some((attribute, force_update)) = attribute_queue.get_next_attribute() {
            // Fetches the initial value. If we're forcing the update, we won't care about it if
            // it's not stored in the cache
            let initial_value = {
                if let Some(value) = self.cache.remove(&attribute) {
                    value
                } else if force_update {
                    0f32
                } else {
                    self.calculate_attribute(&attribute).unwrap_or(0f32)
                }
            };

            // Inserts the updated bonuses into the stack if they're not 0
            if let Some(bonuses) = update_bonuses.remove(&attribute) {
                self.bonuses.insert(attribute, bonuses);
            }

            // If it's forced update, or if the initial value is not equal to the current value.
            // This will coincidentally cache the attribute (if we're not forcing updates)
            if force_update || initial_value != self.get_attribute(&attribute) {
                // Push any bonus attributes that reference the attribute to the queue
                attribute_queue
                    .insert_attriubtes(self.bonuses.get_all_references(&attribute), true);

                //Builds the source for any children bonuses
                let source = attribute.into();

                // Removes any bonuses that are children
                if let Some(children) = self.children.remove(&source) {
                    self.bonuses.remove_source_from(source, children);
                }

                let value = self.get_attribute(&attribute);

                // Checks if there are any child bonuses
                if let Some(mut bonuses) = attribute.get_attribute_bonuses(value) {
                    // Includes child bonuses
                    clone_bonuses(&mut bonuses);

                    // Inserts updated attributes and returns iterator of unique attriubtes
                    let updated_attributes = bonuses
                        .into_iter()
                        .into_group_map_by(|bonus| bonus.get_attribute())
                        .into_iter()
                        .map(|(attribute, mut set)| {
                            // Inserts bonuses into update_bonuses
                            if let Some(update_set) = update_bonuses.get_mut(&attribute) {
                                update_set.append(&mut set);
                            } else {
                                update_bonuses.insert(attribute, set);
                            }
                            attribute
                        })
                        .unique()
                        .collect_vec();

                    // Updates children entry
                    self.children.insert(source, updated_attributes.clone());

                    // Update attribute queue
                    attribute_queue.insert_attriubtes(updated_attributes, false);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bonus::BonusType;

    use super::*;

    #[test]
    fn inserting_attribute_updates_children() {
        let mut compiler = AttributeCompiler::new();

        compiler.insert_bonus(Bonus::dummy(BonusSource::Unique(0)));

        assert_eq!(
            Some(&vec![Attribute::Dummy()]),
            compiler.children.get(&BonusSource::Unique(0))
        );
    }

    #[test]
    fn inserting_attribute_overwrites_children() {
        let mut compiler = AttributeCompiler::new();

        compiler.insert_bonus(Bonus::new(
            Attribute::Dodge(),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1),
            None,
        ));
        compiler.insert_bonus(Bonus::dummy(BonusSource::Unique(1)));

        assert_eq!(
            Some(&vec![Attribute::Dummy()]),
            compiler.children.get(&BonusSource::Unique(1))
        );
    }

    #[test]
    fn getting_bonus_sets_cache() {
        let mut compiler = AttributeCompiler::new();

        compiler.get_attribute(&Attribute::Dummy());

        assert!(compiler.cache.contains_key(&Attribute::Dummy()));
    }
}
