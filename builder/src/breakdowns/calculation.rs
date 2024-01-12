use rust_decimal::Decimal;
use utils::hashmap::IntoGroupedHashMap;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType, Condition, Value},
};

use super::{Breakdowns, EvalBonus};

impl Breakdowns {
    pub(super) fn evaluate_some_condition(&mut self, condition: Option<&Condition>) -> bool {
        condition.map_or(true, |condition| self.evaluate_condition(condition))
    }

    pub(super) fn evaluate_condition(&mut self, condition: &Condition) -> bool {
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

    pub(super) fn evaluate_value(&mut self, value: &Value) -> Decimal {
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
        };

        self.value_cache.insert(value.clone(), result);

        result
    }

    pub(super) fn get_bonus(&mut self, bonus: &Bonus) -> EvalBonus {
        let condition = self.evaluate_some_condition(bonus.condition());
        let value = self.evaluate_value(bonus.value());

        EvalBonus { value, condition }
    }

    /// Calculates and retuns the final value for a given [`Attribute`].
    pub fn get_attribute(&mut self, attribute: impl Into<Attribute>) -> Decimal {
        self.evaluate_value(&Value::Attribute(attribute.into()))
    }

    pub(crate) fn calculate_attribute(&mut self, attribute: &Attribute) -> Option<Decimal> {
        let mut bonuses = self
            .bonuses
            .get(attribute)?
            .clone()
            .into_iter()
            .map(|bonus| (*bonus.bonus_type(), self.get_bonus(&bonus)))
            .filter_map(|(bonus_type, eval)| eval.condition.then_some((bonus_type, eval.value)))
            .into_grouped_hash_map();

        let stacking = bonuses
            .remove(&BonusType::Stacking)
            .map_or(Decimal::ZERO, |i| i.into_iter().sum());

        Some(
            stacking
                + bonuses
                    .into_values()
                    .map(|values| values.into_iter().max().unwrap())
                    .sum::<Decimal>(),
        )
    }
}
