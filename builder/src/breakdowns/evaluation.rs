use std::collections::HashMap;

use rust_decimal::Decimal;
use utils::{from_into::FromInto, hashmap::MapGetOr};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, Condition, Value},
};

use super::{BreakdownCache, Breakdowns, DiceStrategy};

/// Non-Mutating API
impl Breakdowns {
    /// Attempts to return the result of the value from the value cache
    #[must_use]
    pub fn get_value(&self, value: &Value) -> Option<&Decimal> {
        self.cache.value.get(value)
    }

    /// Attempts to return the result of the condition from the condition cache
    #[must_use]
    pub fn get_condition(&self, condition: &Condition) -> Option<bool> {
        self.cache.condition.get(condition).copied()
    }
}

/// Mutable API From methods
impl Breakdowns {
    /// Evaluates a given condition based on values within the current [`Breakdowns`] object.
    pub fn evaluate_condition_from<C>(&mut self, condition: C) -> bool
    where
        C: Into<Condition>,
    {
        self.evaluate_condition(&condition.into())
    }

    /// Evaluates a given value based on values within the current [`Breakdowns`] object.
    pub fn evaluate_value_from<V>(&mut self, value: V) -> Decimal
    where
        V: Into<Value>,
    {
        self.evaluate_value(&value.into())
    }

    /// Evaluates the value of the given attribute. Defaults to [`Decimal::ZERO`] if there are no
    /// bonuses to that attribute.
    pub fn evaluate_attribute_from<A>(&mut self, attribute: A) -> Decimal
    where
        A: Into<Attribute>,
    {
        self.evaluate_attribute(&attribute.into())
    }

    /// Calculates the current value of a given [`Attribute`].
    /// Only takes the highest value of bonuses of the same [`BonusType`], except for
    /// [`BonusType::Stacking`]
    ///
    /// Returns [`Some`] with the resulting value if there are bonuses for it
    /// Returns [`None`] if there are no bonuses available for that [`Attribute`].
    ///
    /// If a [`None`] result should default the value to [`Decimal::ZERO`], then use
    /// [`Breakdowns::evaluate_attribute`]
    ///
    /// [`BonusType`]: crate::bonus::BonusType
    /// [`BonusType::Stacking`]: crate::bonus::BonusType::Stacking
    pub fn calculate_attribute_from<A>(&mut self, attribute: A) -> Option<Decimal>
    where
        A: Into<Attribute>,
    {
        self.calculate_attribute(&attribute.into())
    }
}

/// Mutable API
impl Breakdowns {
    /// Evaluates a given condition based on values within the current [`Breakdowns`] object.
    pub fn evaluate_condition<'a, C>(&mut self, condition: C) -> bool
    where
        C: Into<Option<&'a Condition>>,
    {
        Option::<&'a Condition>::from_into(condition)
            .is_none_or(|cond| self.snapshot().evaluate_condition(cond))
    }

    /// Evaluates a given value based on values within the current [`Breakdowns`] object.
    pub fn evaluate_value(&mut self, value: &Value) -> Decimal {
        self.snapshot().evaluate_value(value)
    }

    /// Evaluates the value of the given attribute. Defaults to [`Decimal::ZERO`] if there are no
    /// bonuses to that attribute.
    pub fn evaluate_attribute(&mut self, attribute: &Attribute) -> Decimal {
        self.snapshot().evaluate_attribute(attribute)
    }

    /// Calculates the current value of a given [`Attribute`].
    /// Only takes the highest value of bonuses of the same [`BonusType`], except for
    /// [`BonusType::Stacking`]
    ///
    /// Returns [`Some`] with the resulting value if there are bonuses for it
    /// Returns [`None`] if there are no bonuses available for that [`Attribute`].
    ///
    /// If a [`None`] result should default the value to [`Decimal::ZERO`], then use
    /// [`Breakdowns::evaluate_attribute`]
    ///
    /// [`BonusType`]: crate::bonus::BonusType
    /// [`BonusType::Stacking`]: crate::bonus::BonusType::Stacking
    pub fn calculate_attribute(&mut self, attribute: &Attribute) -> Option<Decimal> {
        self.snapshot().calculate_attribute(attribute)
    }
}

struct Snapshot<'a> {
    cache: &'a mut BreakdownCache,
    bonuses: &'a HashMap<Attribute, Vec<Bonus>>,
    dice_strategy: DiceStrategy,
}

/// Snapshot Conversion
impl Breakdowns {
    const fn snapshot(&mut self) -> Snapshot<'_> {
        Snapshot {
            cache: &mut self.cache,
            bonuses: &self.bonuses,
            dice_strategy: self.dice_strategy,
        }
    }
}

impl Snapshot<'_> {
    fn calculate_attribute(&mut self, attribute: &Attribute) -> Option<Decimal> {
        let mut map = HashMap::new();
        let mut stacking_bonus = Decimal::ZERO;

        for bonus in self.bonuses.get(attribute)? {
            let apply_value = bonus
                .condition()
                .is_none_or(|cond| self.evaluate_condition(cond));

            if apply_value {
                let value = self.evaluate_value(bonus.value());
                if bonus.bonus_type().is_stacking() {
                    stacking_bonus += value;
                } else {
                    let val = map.get_mut_or(bonus.bonus_type(), Decimal::MIN);
                    *val = value.max(*val);
                }
            }
        }

        Some(stacking_bonus + map.values().sum::<Decimal>())
    }

    fn evaluate_attribute(&mut self, attribute: &Attribute) -> Decimal {
        if let Some(value) = self.cache.attribute.get(attribute) {
            return *value;
        }

        let value = self.calculate_attribute(attribute).unwrap_or(Decimal::ZERO);

        self.cache.attribute.insert(attribute.clone(), value);

        value
    }

    fn evaluate_condition(&mut self, condition: &Condition) -> bool {
        if let Some(value) = self.cache.condition.get(condition) {
            return *value;
        }

        let result = match condition {
            Condition::Constant(value) => return *value,
            Condition::Not(cond) => !self.evaluate_condition(cond),
            Condition::GreaterThan(a, b) => self.evaluate_value(a) > self.evaluate_value(b),
            Condition::GreaterEqualTo(a, b) => self.evaluate_value(a) >= self.evaluate_value(b),
            Condition::LessThan(a, b) => self.evaluate_value(a) < self.evaluate_value(b),
            Condition::LessEqualTo(a, b) => self.evaluate_value(a) <= self.evaluate_value(b),
            Condition::EqualTo(a, b) => self.evaluate_value(a) == self.evaluate_value(b),
            Condition::And(a, b) => self.evaluate_condition(a) && self.evaluate_condition(b),
            Condition::Or(a, b) => self.evaluate_condition(a) || self.evaluate_condition(b),
            Condition::Xor(a, b) => self.evaluate_condition(a) != self.evaluate_condition(b),
        };

        self.cache.condition.insert(condition.clone(), result);
        result
    }

    fn evaluate_value(&mut self, value: &Value) -> Decimal {
        if let Some(value) = self.cache.value.get(value) {
            return *value;
        }

        let result = match value {
            Value::Const(val) => return *val,
            Value::Attribute(attribute) => self.evaluate_attribute(attribute),
            Value::Max(a, b) => self.evaluate_value(a).max(self.evaluate_value(b)),
            Value::Min(a, b) => self.evaluate_value(a).min(self.evaluate_value(b)),
            Value::Floor(val) => self.evaluate_value(val).floor(),
            Value::Abs(val) => self.evaluate_value(val).abs(),
            Value::Ceil(val) => self.evaluate_value(val).ceil(),
            Value::Round(val) => self.evaluate_value(val).round(),
            Value::If {
                condition,
                if_true,
                if_false,
            } => {
                if self.evaluate_condition(condition) {
                    self.evaluate_value(if_true)
                } else {
                    self.evaluate_value(if_false)
                }
            }
            Value::Add(a, b) => self.evaluate_value(a) + self.evaluate_value(b),
            Value::Sub(a, b) => self.evaluate_value(a) - self.evaluate_value(b),
            Value::Mul(a, b) => self.evaluate_value(a) * self.evaluate_value(b),
            Value::Div(a, b) => self.evaluate_value(a) / self.evaluate_value(b),
            Value::Rem(a, b) => self.evaluate_value(a) % self.evaluate_value(b),
            Value::Dice { count, size } => {
                self.evaluate_value(count)
                    * match self.dice_strategy {
                        DiceStrategy::Minimum => Decimal::ONE,
                        DiceStrategy::Average => {
                            (self.evaluate_value(size) + Decimal::ONE) / Decimal::TWO
                        }
                        DiceStrategy::Maximum => self.evaluate_value(size),
                    }
            }
        };

        self.cache.value.insert(value.clone(), result);

        result
    }
}
