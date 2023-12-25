use utils::{float::ErrorMargin, ord::IntoOrdGroupMap};

use crate::{
    attribute::Attribute,
    bonus::{BonusType, Condition, Value},
};

use super::Breakdowns;

impl Breakdowns {
    fn evaluate_condition(&mut self, condition: &Condition) -> bool {
        match condition {
            Condition::Not(cond) => !self.evaluate_condition(cond),
            Condition::GreaterThan(a, b) => self.evaluate_value(a) > self.evaluate_value(b),
            Condition::LessThan(a, b) => self.evaluate_value(a) < self.evaluate_value(b),
            Condition::EqualTo(a, b) => self
                .evaluate_value(a)
                .within_margin(&self.evaluate_value(b)),
            Condition::Constant(value) => *value,
            Condition::And(a, b) => self.evaluate_condition(a) && self.evaluate_condition(b),
            Condition::Or(a, b) => self.evaluate_condition(a) || self.evaluate_condition(b),
            Condition::Xor(a, b) => self.evaluate_condition(a) != self.evaluate_condition(b),
        }
    }

    fn evaluate_value(&mut self, value: &Value) -> f32 {
        match value {
            Value::Value(val) => *val,
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

    pub fn get_attribute(&mut self, attribute: &Attribute) -> f32 {
        if let Some(value) = self.cache.get(attribute) {
            return *value;
        }

        let value = self.calculate_attribute(attribute).unwrap_or(0f32);

        self.cache.insert(*attribute, value);

        value
    }

    pub fn calculate_attribute(&mut self, attribute: &Attribute) -> Option<f32> {
        let bonuses = self.bonuses.get(attribute)?.clone();
        let filtered_bonuses = bonuses
            .into_iter()
            .filter(|bonus| {
                bonus
                    .get_condition()
                    .map_or(true, |condition| self.evaluate_condition(condition))
            })
            .collect::<Vec<_>>();
        let mut map = filtered_bonuses
            .iter()
            .map(|bonus| (bonus.get_type(), self.evaluate_value(bonus.get_value())))
            .into_grouped_ord_map();

        let stacking = map
            .remove(&BonusType::Stacking)
            .map_or(0f32, |i| i.into_iter().sum());

        Some(
            stacking
                + map
                    .into_iter()
                    .map(|(_, mut values)| {
                        let mut value = values.pop().unwrap_or(0f32);
                        for item in values {
                            if value < item {
                                value = item;
                            }
                        }
                        value
                    })
                    .sum::<f32>(),
        )
    }
}
