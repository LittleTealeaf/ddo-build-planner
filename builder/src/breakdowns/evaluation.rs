use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Condition, Value},
};

use super::{Breakdowns, DiceStrategy};

/// Non-Mutating API
impl Breakdowns {
    /// Attempts to return the result of the value from the value cache
    #[must_use]
    pub fn get_value(&self, value: &Value) -> Option<&Decimal> {
        self.value_cache.get(value)
    }

    /// Attempts to return the result of the value from the value cache.
    #[must_use]
    pub fn get_from_value<V>(&self, value: V) -> Option<&Decimal>
    where
        V: Into<Value>,
    {
        self.get_value(&value.into())
    }

    /// Attempts to return the result of the condition from the condition cache
    #[must_use]
    pub fn get_condition(&self, condition: &Condition) -> Option<bool> {
        self.condition_cache.get(condition).copied()
    }

    /// Attempts to return the result of the condition from the condition cache
    pub fn get_from_condition<C>(&self, condition: C) -> Option<bool>
    where
        C: Into<Condition>,
    {
        self.get_condition(&condition.into())
    }
}

/// Mutable API
impl Breakdowns {
    pub fn evaluate_from_condition<C>(&mut self, condition: C) -> bool
    where
        C: Into<Condition>,
    {
        self.evaluate_condition(&condition.into())
    }

    pub fn evaluate_from_value<V>(&mut self, value: V) -> Decimal
    where
        V: Into<Value>,
    {
        self.evaluate_value(&value.into())
    }

    pub fn evaluate_from_attribute<A>(&mut self, attribute: A) -> Decimal
    where
        A: Into<Attribute>,
    {
        self.calculate_from_attribute(attribute)
            .unwrap_or(Decimal::ZERO)
    }

    pub fn calculate_from_attribute<A>(&mut self, attribute: A) -> Option<Decimal>
    where
        A: Into<Attribute>,
    {
        self.calculate_attribute(&attribute.into())
    }

    pub fn evaluate_condition(&mut self, condition: &Condition) -> bool {
        if let Some(value) = self.condition_cache.get(condition) {
            return *value;
        }

        let result = match condition {
            Condition::Constant(value) => return *value,
            Condition::Not(cond) => !self.evaluate_condition(cond),
            Condition::GreaterThan(a, b) => self.evaluate_value(a) > self.evaluate_value(b),
            Condition::LessThan(a, b) => self.evaluate_value(a) < self.evaluate_value(b),
            Condition::EqualTo(a, b) => self.evaluate_value(a) == self.evaluate_value(b),
            Condition::And(a, b) => self.evaluate_condition(a) && self.evaluate_condition(b),
            Condition::Or(a, b) => self.evaluate_condition(a) || self.evaluate_condition(b),
            Condition::Xor(a, b) => self.evaluate_condition(a) != self.evaluate_condition(b),
        };

        self.condition_cache.insert(condition.clone(), result);
        result
    }

    pub fn evaluate_value(&mut self, value: &Value) -> Decimal {
        if let Some(value) = self.value_cache.get(value) {
            return *value;
        }

        let result = match value {
            Value::Const(val) => return *val,
            Value::Attribute(attribute) => {
                self.calculate_attribute(attribute).unwrap_or(Decimal::ZERO)
            }
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

        self.value_cache.insert(value.clone(), result);

        result
    }

    /// Evaluates the value of the given attribute. Defaults to [`Decimal::ZERO`] if there are no
    /// bonuses to that attribute.
    pub fn evaluate_attribute(&mut self, attribute: &Attribute) -> Decimal {
        self.calculate_attribute(attribute).unwrap_or(Decimal::ZERO)
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
        let mut map = HashMap::new();
        let mut stacking = Decimal::ZERO;

        for bonus in self.bonuses.get(attribute)?.clone() {
            let condition = bonus
                .condition()
                .map_or(true, |cond| self.evaluate_condition(cond));

            if condition {
                let value = self.evaluate_value(bonus.value());
                if bonus.bonus_type().is_stacking() {
                    stacking += value;
                } else {
                    map.insert(
                        *bonus.bonus_type(),
                        value.max(*map.get(bonus.bonus_type()).unwrap_or(&Decimal::MIN)),
                    );
                }
            }
        }

        Some(stacking + map.values().sum::<Decimal>())
    }
}
