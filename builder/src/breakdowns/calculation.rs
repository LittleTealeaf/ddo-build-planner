use im::HashMap;
use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Condition, Value},
};

use super::{Breakdowns, DiceStrategy};

impl Breakdowns {
    /// Calculates and retuurns the final value for a given [`Attribute`].
    pub fn get_attribute_old<A>(&mut self, attribute: A) -> Decimal
    where
        A: Into<Attribute>,
    {
        self.evaluate_value_old(&Value::Attribute(attribute.into()))
    }

    pub(super) fn evaluate_some_condition_old(&mut self, condition: Option<&Condition>) -> bool {
        condition.map_or(true, |condition| self.evaluate_condition_old(condition))
    }

    pub(super) fn evaluate_condition_old(&mut self, condition: &Condition) -> bool {
        if let Some(value) = self.condition_cache.get(condition) {
            return *value;
        }

        let result = match condition {
            Condition::Constant(value) => return *value,
            Condition::Not(cond) => !self.evaluate_condition_old(cond),
            Condition::GreaterThan(a, b) => self.evaluate_value_old(a) > self.evaluate_value_old(b),
            Condition::LessThan(a, b) => self.evaluate_value_old(a) < self.evaluate_value_old(b),
            Condition::EqualTo(a, b) => self.evaluate_value_old(a) == self.evaluate_value_old(b),
            Condition::And(a, b) => self.evaluate_condition_old(a) && self.evaluate_condition_old(b),
            Condition::Or(a, b) => self.evaluate_condition_old(a) || self.evaluate_condition_old(b),
            Condition::Xor(a, b) => self.evaluate_condition_old(a) != self.evaluate_condition_old(b),
        };

        self.condition_cache.insert(condition.clone(), result);
        result
    }

    pub(super) fn evaluate_value_old(&mut self, value: &Value) -> Decimal {
        if let Some(value) = self.value_cache.get(value) {
            return *value;
        }

        let result = match value {
            Value::Const(val) => return *val,
            Value::Attribute(attribute) => {
                self.calculate_attribute_old(attribute).unwrap_or(Decimal::ZERO)
            }
            Value::Max(a, b) => self.evaluate_value_old(a).max(self.evaluate_value_old(b)),
            Value::Min(a, b) => self.evaluate_value_old(a).min(self.evaluate_value_old(b)),
            Value::Floor(val) => self.evaluate_value_old(val).floor(),
            Value::Abs(val) => self.evaluate_value_old(val).abs(),
            Value::Ceil(val) => self.evaluate_value_old(val).ceil(),
            Value::Round(val) => self.evaluate_value_old(val).round(),
            Value::If {
                condition,
                if_true,
                if_false,
            } => {
                if self.evaluate_condition_old(condition) {
                    self.evaluate_value_old(if_true)
                } else {
                    self.evaluate_value_old(if_false)
                }
            }
            Value::Add(a, b) => self.evaluate_value_old(a) + self.evaluate_value_old(b),
            Value::Sub(a, b) => self.evaluate_value_old(a) - self.evaluate_value_old(b),
            Value::Mul(a, b) => self.evaluate_value_old(a) * self.evaluate_value_old(b),
            Value::Div(a, b) => self.evaluate_value_old(a) / self.evaluate_value_old(b),
            Value::Rem(a, b) => self.evaluate_value_old(a) % self.evaluate_value_old(b),
            Value::Dice { count, size } => {
                self.evaluate_value_old(count)
                    * match self.dice_strategy {
                        DiceStrategy::Minimum => Decimal::ONE,
                        DiceStrategy::Average => {
                            (self.evaluate_value_old(size) + Decimal::ONE) / Decimal::TWO
                        }
                        DiceStrategy::Maximum => self.evaluate_value_old(size),
                    }
            }
        };

        self.value_cache.insert(value.clone(), result);

        result
    }

    pub(crate) fn calculate_attribute_old(&mut self, attribute: &Attribute) -> Option<Decimal> {
        let mut map = HashMap::new();
        let mut stacking = Decimal::ZERO;

        for bonus in self.bonuses.get(attribute)?.clone() {
            if self.evaluate_some_condition_old(bonus.condition()) {
                let value = self.evaluate_value_old(bonus.value());
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
