use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusValue, Condition},
    utils::EnumBinaryMap,
};

use super::Compiler;

// Supporting Functions
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

    /// Calculates an attribute.
    ///
    /// Does not insert that attribute into the cache.
    pub fn calculate_attribute(&mut self, attribute: &Attribute) -> Option<f32> {
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
