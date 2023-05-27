//! Handles the compilation and calculations of [`Bonuses`].
//!
//! [`Bonuses`]: crate::bonus::Bonus

mod attribute_queue;

use enum_map::IntoIter;
use itertools::Itertools;

use crate::{
    attribute::{self, Attribute},
    bonus::{Bonus, BonusSource, BonusValue, Condition},
    utils::EnumBinaryMap,
};

use self::attribute_queue::AttributeQueue;
pub struct Compiler {
    bonuses: EnumBinaryMap<Attribute, Vec<Bonus>>,
    cache: EnumBinaryMap<Attribute, f32>,
    children: EnumBinaryMap<BonusSource, Vec<Attribute>>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            bonuses: Default::default(),
            cache: Default::default(),
            children: Default::default(),
        }
    }
}

// Public Interface
impl Compiler {
    pub fn shrink_to_fit(&mut self) {
        self.bonuses.shrink_to_fit();
        self.cache.shrink_to_fit();
        self.children.shrink_to_fit();
    }

    pub fn get_attribute(&self, attribute: &Attribute) -> f32 {
        self.cache
            .get(attribute)
            .map(|value| *value)
            .unwrap_or_else(|| self.calculate_attribute(attribute).unwrap_or(0f32))
    }

    pub fn get_attribute_cached(&mut self, attribute: &Attribute) -> f32 {
        self.cache
            .get(attribute)
            .map(|value| *value)
            .unwrap_or_else(|| {
                let value = self.calculate_attribute_mut(attribute).unwrap_or(0f32);
                self.cache.insert(*attribute, value);
                value
            })
    }
}

// Calculating Attributes
impl Compiler {
    fn check_condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::Has(attr) => self.get_attribute(&attr) > 0f32,
            Condition::NotHave(attr) => self.get_attribute(&attr) == 0f32,
            Condition::Max(attr, val) => self.get_attribute(&attr) <= val,
            Condition::Min(attr, val) => self.get_attribute(&attr) >= val,
            Condition::Eq(attr, val) => self.get_attribute(&attr) == val,
            Condition::NotEq(attr, val) => self.get_attribute(&attr) != val,
            Condition::Any(set) => set.into_iter().any(|cond| self.check_condition(cond)),
            Condition::All(set) => set.into_iter().all(|cond| self.check_condition(cond)),
        }
    }

    fn calculate_attribute(&self, attribute: &Attribute) -> Option<f32> {
        Some(
            EnumBinaryMap::from(self.bonuses.get(attribute)?.iter().filter_map(|bonus| {
                bonus
                    .get_condition()
                    .map(|condition| self.check_condition(condition))
                    .unwrap_or(true)
                    .then(|| {
                        (
                            bonus.get_bous_type(),
                            match bonus.get_bonus_value() {
                                BonusValue::Value(val) => val,
                                BonusValue::Indirect(attribute) => self.get_attribute(&attribute),
                                BonusValue::IndirectScaled(attribute, scale) => {
                                    self.get_attribute(&attribute) * scale
                                }
                            },
                        )
                    })
            }))
            .into_iter()
            .map(|(bonus_type, mut items)| {
                let mut value = items.pop().unwrap_or(0f32);
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
                return value;
            })
            .sum(),
        )
    }

    fn check_condition_mut(&mut self, condition: Condition) -> bool {
        match condition {
            Condition::Has(attr) => self.get_attribute_cached(&attr) > 0f32,
            Condition::NotHave(attr) => self.get_attribute_cached(&attr) == 0f32,
            Condition::Max(attr, val) => self.get_attribute_cached(&attr) <= val,
            Condition::Min(attr, val) => self.get_attribute_cached(&attr) >= val,
            Condition::Eq(attr, val) => self.get_attribute_cached(&attr) == val,
            Condition::NotEq(attr, val) => self.get_attribute_cached(&attr) != val,
            Condition::Any(set) => set.into_iter().any(|cond| self.check_condition_mut(cond)),
            Condition::All(set) => set.into_iter().all(|cond| self.check_condition_mut(cond)),
        }
    }

    fn calculate_attribute_mut(&mut self, attribute: &Attribute) -> Option<f32> {
        Some(
            EnumBinaryMap::from(self.bonuses.get(attribute)?.clone().into_iter().filter_map(
                |bonus| {
                    bonus
                        .get_condition()
                        .map(|condition| self.check_condition_mut(condition))
                        .unwrap_or(true)
                        .then(|| {
                            (
                                bonus.get_bous_type(),
                                match bonus.get_bonus_value() {
                                    BonusValue::Value(val) => val,
                                    BonusValue::Indirect(attribute) => {
                                        self.get_attribute_cached(&attribute)
                                    }
                                    BonusValue::IndirectScaled(attribute, scale) => {
                                        self.get_attribute_cached(&attribute) * scale
                                    }
                                },
                            )
                        })
                },
            ))
            .into_iter()
            .map(|(bonus_type, mut items)| {
                let mut value = items.pop().unwrap_or(0f32);
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
                return value;
            })
            .sum(),
        )
    }
}

// Helper functions for inserting bonuses
impl Compiler {
    fn remove_by_source(&mut self, source: BonusSource, children: Vec<Attribute>) {
        for child in children.into_iter() {
            if let Some(set) = self.bonuses.get_mut(&child) {
                set.iter()
                    .enumerate()
                    .filter(|(_, item)| item.get_source().eq(&source))
                    .map(|(i, _)| i)
                    .rev()
                    .collect_vec()
                    .into_iter()
                    .for_each(|i| {
                        set.swap_remove(i);
                    });
            }
        }
    }

    fn condition_has_source(condition: &Condition, source: Attribute) -> bool {
        match condition {
            Condition::Has(attr)
            | Condition::NotHave(attr)
            | Condition::Max(attr, _)
            | Condition::Min(attr, _)
            | Condition::Eq(attr, _)
            | Condition::NotEq(attr, _) => source.eq(attr),
            Condition::Any(set) | Condition::All(set) => set
                .iter()
                .any(|item| Compiler::condition_has_source(item, source)),
        }
    }

    fn get_all_references(&self, attribute: &Attribute) -> Vec<Attribute> {
        self.bonuses
            .iter()
            .filter_map(|(key, set)| {
                {
                    set.iter().any(|bonus| {
                        bonus
                            .get_condition()
                            .map(|condition| Compiler::condition_has_source(&condition, *attribute))
                            .unwrap_or(false)
                            || match bonus.get_bonus_value() {
                                BonusValue::Value(_) => false,
                                BonusValue::Indirect(attr)
                                | BonusValue::IndirectScaled(attr, _) => attribute.eq(&attr),
                            }
                    })
                }
                .then(|| key)
            })
            .collect()
    }
}

// Inserting Attributes
impl Compiler {
    pub fn insert_bonus(&mut self, bonus: Bonus) {
        self.insert_bonuses(vec![bonus])
    }

    pub fn insert_bonuses(&mut self, bonuses: Vec<Bonus>) {
        // Create attribute queue
        let mut attribute_queue = AttributeQueue::default();

        attribute_queue.insert(bonuses.iter().map(Bonus::get_attribute).collect(), false);

        // Update children for the given source(s)
        EnumBinaryMap::from(
            bonuses
                .iter()
                .map(|bonus| (bonus.get_source(), bonus.get_attribute())),
        )
        .into_iter()
        .for_each(|(source, set)| {
            if let Some(children) = self.children.insert(source, set) {
                self.remove_by_source(source, children);
            }
        });

        // Initialize the update bonuses
        let mut update_bonuses = EnumBinaryMap::from(
            bonuses
                .into_iter()
                .map(|bonus| (bonus.get_attribute(), bonus)),
        );

        while let Some((attribute, force_update)) = attribute_queue.get_next_attribute() {
            let initial_value = {
                if let Some(value) = self.cache.remove(&attribute) {
                    value
                } else if force_update {
                    0f32
                } else {
                    self.calculate_attribute(&attribute).unwrap_or(0f32)
                }
            };

            if let Some(mut bonuses) = update_bonuses.remove(&attribute) {
                self.bonuses
                    .get_mut_or_default(&attribute)
                    .append(&mut bonuses);
            }

            if force_update || initial_value != self.get_attribute_cached(&attribute) {
                // Push any bonus attributes that reference this attribute to the queue
                attribute_queue.insert(self.get_all_references(&attribute), true);

                // Builds the source for any children bonuses
                let source = attribute.into();

                // removes any bonuses that are children
                if let Some(children) = self.children.remove(&source) {
                    self.remove_by_source(source, children);
                }

                let value = self.get_attribute_cached(&attribute);

                if let Some(bonuses) = attribute.get_bonuses(value) {
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

#[cfg(test)]
mod tests {
    use crate::{attribute::types::Ability, bonus::BonusType};

    use super::*;

    #[test]
    fn inserting_attribute_updates_children() {
        let mut compiler = Compiler::default();

        compiler.insert_bonus(Bonus::dummy(0.into()));

        assert_eq!(
            Some(&vec![Attribute::Dummy]),
            compiler.children.get(&(0.into()))
        )
    }

    #[test]
    fn inserting_attributes_overwrites_children() {
        let mut compiler = Compiler::default();

        compiler.insert_bonus(Bonus::new(
            Attribute::Ability(Ability::Wisdom),
            BonusType::Stacking,
            1f32.into(),
            BonusSource::Custom(1),
            None,
        ));

        compiler.insert_bonus(Bonus::dummy(BonusSource::Custom(1)));

        assert_eq!(
            Some(&vec![Attribute::Dummy]),
            compiler.children.get(&(1.into()))
        );
    }
}
