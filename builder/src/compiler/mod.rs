//! Handles the compilation and calculations of [`Bonuses`].
//!
//! [`Bonuses`]: crate::bonus::Bonus

mod attribute_queue;

use itertools::Itertools;

use crate::{
    attribute::{Attribute, DefaultBonuses},
    bonus::{Bonus, BonusSource, BonusValue, CloneBonus, Condition},
    utils::EnumBinaryMap,
};

use self::attribute_queue::AttributeQueue;

/// Compiles and calculates attribut values from a set of [`Bonus`] entries.
///
/// Internally, this uses [`EnumBinaryMaps`] to efficiently store bonuses in a HashMap structure without the need of deriving [`Hash`].
///
/// This will handle any bonuses that different attributes may give (such as [`Attribute::Ability`] giving bonuses to [`Attribute::AbilityModifier`]), as well as cloned bonuses (such as [`Ability::All`] being split off into each of the abilities)
///
/// Note that the compiler must be mutable for most of it's publicly-facing functions
///
/// # Examples
///
/// ```
/// use builder::{
///     attribute::{
///         Attribute,
///         types::{
///             Sheltering
///         }
///     },
///     bonus::{Bonus, BonusSource, Condition, BonusType},
///     compiler::Compiler
/// };
///
/// let mut compiler = Compiler::default();
///
/// compiler.add_bonus(Bonus::new(Attribute::Sheltering(Sheltering::Magical), BonusType::Stacking, 5f32.into(), BonusSource::Custom(0), None));
///
/// assert_eq!(5f32, compiler.get_attribute(&Attribute::Sheltering(Sheltering::Magical)));
/// ```
///
///
/// [`EnumBinaryMaps`]: crate::utils::EnumBinaryMap
/// [`Bonus`]: crate::bonus::Bonus
/// [`Ability::All`]: crate::attribute::types::Ability::All
pub struct Compiler {
    bonuses: EnumBinaryMap<Attribute, Vec<Bonus>>,
    cache: EnumBinaryMap<Attribute, f32>,
    children: EnumBinaryMap<BonusSource, Vec<Attribute>>,
}

impl Default for Compiler {
    fn default() -> Self {
        let mut new = Self {
            bonuses: EnumBinaryMap::default(),
            cache: EnumBinaryMap::default(),
            children: EnumBinaryMap::default(),
        };

        new.add_bonuses(Attribute::get_default_bonuses());

        new
    }
}

// public functions
impl Compiler {
    /// Returns the value of an attribute.
    ///
    /// If the attribute has no bonuses, then it will return `0f32`
    pub fn get_attribute(&mut self, attribute: &Attribute) -> f32 {
        // First try the cache
        if let Some(value) = self.cache.get(attribute) {
            return *value;
        }

        // Otherwise, calculate the value
        let value = self.calculate_attribute(attribute).unwrap_or(0f32);
        // store in cache
        self.cache.insert(*attribute, value);

        // Return the value
        value
    }

    /// Returns all attributes that have bonuses in the compiler.
    pub fn get_all_attributes(&mut self) -> Vec<(Attribute, f32)> {
        let attributes = self.bonuses.iter().map(|(attr, _)| attr).collect_vec();
        attributes
            .into_iter()
            .map(|attr| (attr, self.get_attribute(&attr)))
            .collect()
    }

    /// Adds one bonus to the compiler set.
    pub fn add_bonus(&mut self, bonus: Bonus) {
        self.add_bonuses(vec![bonus]);
    }

    /// Adds a set of bonuses to the compiler set.
    pub fn add_bonuses(&mut self, bonuses: Vec<Bonus>) {
        self.insert_bonuses(bonuses);
    }
}

// Calculating Functions
impl Compiler {
    fn check_condition(&mut self, condition: Condition) -> bool {
        match condition {
            Condition::Has(attr) => self.get_attribute(&attr) > 0f32,
            Condition::NotHave(attr) => self.get_attribute(&attr) <= 0f32,
            Condition::Max(attr, val) => self.get_attribute(&attr) <= val,
            Condition::Min(attr, val) => self.get_attribute(&attr) >= val,
            Condition::Eq(attr, val) => self.get_attribute(&attr) == val,
            Condition::NotEq(attr, val) => self.get_attribute(&attr) != val,
            Condition::Any(set) => set.into_iter().any(|cond| self.check_condition(cond)),
            Condition::All(set) => set.into_iter().all(|cond| self.check_condition(cond)),
            Condition::GreaterThan(a, b) => self.get_attribute(&a) > self.get_attribute(&b),
            Condition::LessThan(a, b) => self.get_attribute(&a) < self.get_attribute(&b),
            Condition::EqualTo(a, b) => self.get_attribute(&a) == self.get_attribute(&b),
            Condition::Not(condition) => !self.check_condition(*condition),
            Condition::NotAll(conditions) => conditions
                .into_iter()
                .any(|cond| !self.check_condition(cond)),
            Condition::None(conditions) => conditions
                .into_iter()
                .all(|cond| !self.check_condition(cond)),
            Condition::NotEqualTo(a, b) => self.get_attribute(&a) != self.get_attribute(&b),
        }
    }

    fn calculate_value(&mut self, value: BonusValue) -> f32 {
        match value {
            BonusValue::Value(val) => val,
            BonusValue::FromAttribute(attribute) => self.get_attribute(&attribute),
            BonusValue::ScaleAttribute(attribute, scale) => self.get_attribute(&attribute) * scale,
        }
    }

    fn calculate_attribute(&mut self, attribute: &Attribute) -> Option<f32> {
        // Collect valid bonuses that pass their conditions into a list of (type, value) tuples
        let valid_bonuses = self
            .bonuses
            .get(attribute)?
            .clone()
            .into_iter()
            .filter_map(|bonus| {
                bonus
                    .get_condition()
                    .map(|condition| self.check_condition(condition))
                    .unwrap_or(true)
                    .then(|| (bonus.get_type(), self.calculate_value(bonus.get_value())))
            });

        // Collect each type into a vec with EnumBinaryMap
        let map = EnumBinaryMap::from(valid_bonuses);

        // flatten each type into a number
        let final_values = map.into_iter().map(|(bonus_type, mut items)| {
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
            value
        });

        Some(final_values.sum())
    }
}

// Inserting Bonuses
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

    fn insert_bonuses(&mut self, mut bonuses: Vec<Bonus>) {
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
