use rust_decimal::Decimal;
use utils::{hashmap::IntoGroupedHashMap, ord::IntoOrdGroupMap};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType, Condition, Value},
};

use super::{Breakdowns, EvalBonus};

impl Breakdowns {
    pub(super) fn evaluate_condition(&mut self, condition: &Condition) -> bool {
        match condition {
            Condition::Not(cond) => !self.evaluate_condition(cond),
            Condition::GreaterThan(a, b) => self.evaluate_value(a) > self.evaluate_value(b),
            Condition::LessThan(a, b) => self.evaluate_value(a) < self.evaluate_value(b),
            Condition::EqualTo(a, b) => self.evaluate_value(a) == self.evaluate_value(b),
            Condition::Constant(value) => *value,
            Condition::And(a, b) => self.evaluate_condition(a) && self.evaluate_condition(b),
            Condition::Or(a, b) => self.evaluate_condition(a) || self.evaluate_condition(b),
            Condition::Xor(a, b) => self.evaluate_condition(a) != self.evaluate_condition(b),
        }
    }

    pub(super) fn evaluate_value(&mut self, value: &Value) -> Decimal {
        match value {
            Value::Const(val) => *val,
            Value::Attribute(attribute) => self.get_attribute(attribute),
            Value::Max(a, b) => self.evaluate_value(a).max(self.evaluate_value(b)),
            Value::Min(a, b) => self.evaluate_value(a).min(self.evaluate_value(b)),
            Value::Floor(val) => self.evaluate_value(val).floor(),
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
        }
    }

    pub(super) fn get_bonus(&mut self, bonus: &Bonus) -> EvalBonus {
        if let Some(eval) = self.bonus_cache.get(bonus) {
            return *eval;
        }

        let bonus_eval = self.calculate_bonus(bonus);

        self.bonus_cache.insert(bonus.clone(), bonus_eval);

        bonus_eval
    }

    pub(super) fn calculate_bonus(&mut self, bonus: &Bonus) -> EvalBonus {
        let value = self.evaluate_value(bonus.get_value());
        let condition = bonus
            .get_condition()
            .map_or(true, |condition| self.evaluate_condition(condition));

        EvalBonus { value, condition }
    }

    /// Calculates and retuns the final value for a given [`Attribute`].
    pub fn get_attribute(&mut self, attribute: &Attribute) -> Decimal {
        if let Some(value) = self.attribute_cache.get(attribute) {
            return *value;
        }

        let value = self
            .calculate_attribute(*attribute)
            .unwrap_or(Decimal::ZERO);

        self.attribute_cache.insert(*attribute, value);

        value
    }

    pub(crate) fn calculate_attribute(&mut self, attribute: Attribute) -> Option<Decimal> {
        let mut bonuses = self
            .bonuses
            .get(&attribute)?
            .clone()
            .into_iter()
            .map(|bonus| (*bonus.get_type(), self.get_bonus(&bonus)))
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
