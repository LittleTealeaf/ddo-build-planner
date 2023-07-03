use itertools::Itertools;
use utils::ord::IntoOrdGroupMap;

use crate::{
    attribute::Attribute,
    bonus::{Condition, Value},
};

use super::Compiler;

// Supporting Functions
impl Compiler {
    fn check_condition(&mut self, condition: &Condition) -> bool {
        let check_condition = |cond: &Condition| self.check_condition(cond);

        match condition {
            Condition::Not(cond) => self.check_condition(cond),
            Condition::GreaterThan(a, b) => self.calculate_value(a) > self.calculate_value(b),
            Condition::LessThan(a, b) => self.calculate_value(a) < self.calculate_value(b),
            Condition::EqualTo(a, b) => self.calculate_value(a) == self.calculate_value(b),
            Condition::NotEqualTo(a, b) => self.calculate_value(a) != self.calculate_value(b),
            Condition::Any(conds) => conds.iter().any(check_condition),
            Condition::All(conds) => conds.iter().all(check_condition),
            Condition::NotAny(conds) => !conds.iter().any(check_condition),
            Condition::NotAll(conds) => !conds.iter().all(check_condition),
        }
    }

    fn calculate_value(&mut self, value: &Value) -> f32 {
        match value {
            Value::Value(val) => *val,
            Value::Attribute(attribute) => self.get_attribute(attribute),
            Value::Sum(vals) => vals.into_iter().map(|val| self.calculate_value(val)).sum(),
            Value::Product(vals) => vals.iter().map(|val| self.calculate_value(val)).product(),
            Value::Min(vals) => {
                let mut iter = vals.into_iter();

                if let Some(first) = iter.next() {
                    let mut min = self.calculate_value(first);

                    for item in iter {
                        let val = self.calculate_value(item);
                        if min > val {
                            min = val;
                        }
                    }
                    min
                } else {
                    0f32
                }
            }
            Value::Max(vals) => {
                let mut iter = vals.into_iter();

                if let Some(first) = iter.next() {
                    let mut max = self.calculate_value(first);

                    for item in iter {
                        let val = self.calculate_value(item);
                        if max < val {
                            max = val;
                        }
                    }
                    max
                } else {
                    0f32
                }
            }
            Value::Floor(val) => self.calculate_value(val).floor(),
        }
    }
}

/// Implementations for getting values from the compiler.
impl Compiler {
    /// Returns the calculated value of the given [`Attribute`].
    ///
    /// If the [`Attribute`] is not currently tracked in the system, then this will simply return
    /// `0f32`.
    ///
    /// This function will first check if the value can be found in the cache. If it can't, then it
    /// will use [`Compiler::calculate_attribute`] to get the calculated value, and store that in the
    /// cache.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{
    ///     compiler::Compiler,
    ///     attribute::{
    ///         Attribute,
    ///         types::Ability
    ///     }
    /// };
    ///
    /// let mut compiler = Compiler::default();
    /// assert!(compiler.get_attribute(&Attribute::SpellResistance) == 0f32);
    ///
    /// // Note that attributes like Ability Scores are automatically inserted on creation.
    ///
    /// assert!(compiler.get_attribute(&Attribute::Ability(Ability::Strength)) == 8f32);
    /// ```
    ///
    /// [`Compiler::calculate_attribute`]: crate::compiler::Compiler::calculate_attribute()
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
        let attributes = self.bonuses.iter().map(|(key, _)| *key).collect_vec();
        attributes
            .into_iter()
            .map(|attr| (attr, self.get_attribute(&attr)))
            .collect()
    }

    /// Calculates the total value of valid bonuses for a particular attribute.
    ///
    /// **Note**: This does not update the cache. For most cases, it is advisable to
    /// use [`Condition::get_attribute`].
    ///
    /// If there are no bonuses that apply to that attribute in the compiler, this returns
    /// `None`. If there are no *active* bonuses, this will return `Some(0f32)`.
    ///
    /// Note that *active* bonsues are bonuses who either have no [`Condition`] or a true
    /// [`Condition`].
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use builder::{
    ///     compiler::Compiler,
    ///     attribute::{
    ///         Attribute,
    ///         types::Ability
    ///     }
    /// };
    ///
    /// let mut compiler = Compiler::default();
    /// assert_eq!(None, compiler.calculate_attribute(&Attribute::SpellResistance));
    ///
    /// // Note that attributes like Ability Scores are automatically inserted on creation.
    ///
    /// assert!(compiler.calculate_attribute(&Attribute::Ability(Ability::Charisma)).is_some());
    /// ```
    ///
    /// [`Condition::get_attribute`]: crate::compiler::Compiler::get_attribute()
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
                    .map(|condition| self.check_condition(&condition))
                    .unwrap_or(true)
                    .then(|| (bonus.get_type(), self.calculate_value(&bonus.get_value())))
            });

        // Collect each type into a vec with EnumBinaryMap
        let map = valid_bonuses.into_grouped_ord_map();

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
