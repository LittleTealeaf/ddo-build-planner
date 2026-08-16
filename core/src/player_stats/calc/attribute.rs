use std::collections::HashMap;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    attribute::Attribute,
    bonus::{BonusCondition, BonusType, BonusValue},
    player_stats::calc::StatsCalculator,
};

impl StatsCalculator<'_> {
    pub fn evaluate_attribute(&mut self, attribute: Attribute, snapshot: Option<u32>) -> Decimal {
        self.get_value(&BonusValue::Attribute(attribute), snapshot)
    }

    pub fn get_condition(&mut self, condition: &BonusCondition, snapshot: Option<u32>) -> bool {
        if let BonusCondition::Constant(val) = &condition {
            return *val;
        }

        if let Some(value) = self.cache.conditions.get(&(condition.clone(), snapshot)) {
            return *value;
        }
        if let BonusCondition::Constant(val) = condition {
            return *val;
        }

        let value = self.calculate_condition(condition, snapshot);

        self.cache
            .conditions
            .insert((condition.clone(), snapshot), value);
        value
    }

    fn calculate_condition(&mut self, condition: &BonusCondition, snapshot: Option<u32>) -> bool {
        match condition {
            BonusCondition::Not(cond) => !self.get_condition(cond, snapshot),
            BonusCondition::GreaterThan(val1, val2) => {
                self.get_value(val1, snapshot) > self.get_value(val2, snapshot)
            }
            BonusCondition::GreaterEqualTo(val1, val2) => {
                self.get_value(val1, snapshot) >= self.get_value(val2, snapshot)
            }
            BonusCondition::LessThan(val1, val2) => {
                self.get_value(val1, snapshot) < self.get_value(val2, snapshot)
            }
            BonusCondition::LessEqualTo(val1, val2) => {
                self.get_value(val1, snapshot) <= self.get_value(val2, snapshot)
            }
            BonusCondition::EqualTo(val1, val2) => {
                self.get_value(val1, snapshot) == self.get_value(val2, snapshot)
            }
            BonusCondition::And(cond1, cond2) => {
                self.get_condition(cond1, snapshot) && self.get_condition(cond2, snapshot)
            }
            BonusCondition::Or(cond1, cond2) => {
                self.get_condition(cond1, snapshot) || self.get_condition(cond2, snapshot)
            }
            BonusCondition::Xor(cond1, cond2) => {
                self.get_condition(cond1, snapshot) ^ self.get_condition(cond2, snapshot)
            }
            BonusCondition::Constant(val) => *val,
        }
    }

    pub fn get_value(&mut self, value: &BonusValue, snapshot: Option<u32>) -> Decimal {
        if let Some(val) = self.cache.values.get(&(value.clone(), snapshot)) {
            return *val;
        }
        if let BonusValue::Const(val) = value {
            return *val;
        }

        let val = self.calculate_value(value, snapshot);

        self.cache.values.insert((value.clone(), snapshot), val);

        val
    }

    fn calculate_value(&mut self, value: &BonusValue, snapshot: Option<u32>) -> Decimal {
        match value {
            BonusValue::Attribute(attribute) => self.calculate_attribute(attribute, snapshot),
            BonusValue::Snapshot(snap, bonus_value) => self.get_value(bonus_value, Some(*snap)),
            BonusValue::Min(val1, val2) => self
                .get_value(val1, snapshot)
                .min(self.get_value(val2, snapshot)),
            BonusValue::Max(val1, val2) => self
                .get_value(val1, snapshot)
                .max(self.get_value(val2, snapshot)),
            BonusValue::Floor(val1) => self.get_value(val1, snapshot).floor(),
            BonusValue::Ceil(val1) => self.get_value(val1, snapshot).ceil(),
            BonusValue::Round(val1) => self.get_value(val1, snapshot).round(),
            BonusValue::Abs(val1) => self.get_value(val1, snapshot).abs(),
            BonusValue::Add(val1, val2) => {
                self.get_value(val1, snapshot) + self.get_value(val2, snapshot)
            }
            BonusValue::Sub(val1, val2) => {
                self.get_value(val1, snapshot) - self.get_value(val2, snapshot)
            }
            BonusValue::Mul(val1, val2) => {
                self.get_value(val1, snapshot) * self.get_value(val2, snapshot)
            }
            BonusValue::Div(val1, val2) => {
                let denominator = self.get_value(val2, snapshot);
                if denominator == Decimal::ZERO {
                    return Decimal::ZERO;
                }
                self.get_value(val1, snapshot) / denominator
            }
            BonusValue::Rem(val1, val2) => {
                self.get_value(val1, snapshot) % self.get_value(val2, snapshot)
            }
            BonusValue::If {
                condition,
                if_true,
                if_false,
            } => {
                if self.get_condition(condition, snapshot) {
                    self.get_value(if_true, snapshot)
                } else {
                    self.get_value(if_false, snapshot)
                }
            }
            BonusValue::Dice { count, size } => {
                ((self.get_value(size, snapshot) + dec!(1)) / dec!(2))
                    * self.get_value(count, snapshot)
            }
            BonusValue::Const(decimal) => *decimal,
        }
    }

    fn calculate_attribute(&mut self, attribute: &Attribute, snapshot: Option<u32>) -> Decimal {
        let mut values = HashMap::new();
        let multiply = attribute.multiplicative();
        let mut stacking = if multiply { dec!(1) } else { dec!(0) };

        for bonus in self.bonuses.get_snapshot_bonuses(attribute, snapshot) {
            if let Some(condition) = bonus.show_condition() {
                if !self.get_condition(condition, snapshot) {
                    continue;
                }
            }

            if let Some(condition) = bonus.condition() {
                if !self.get_condition(condition, snapshot) {
                    continue;
                }
            }

            let value = self.get_value(bonus.value(), snapshot);
            match bonus.bonus_type() {
                BonusType::Stacking => {
                    if multiply {
                        stacking *= dec!(1) - value;
                    } else {
                        stacking += value;
                    }
                }
                ty => {
                    values
                        .entry(ty)
                        .and_modify(|val: &mut Decimal| {
                            if &value > val {
                                *val = value;
                            }
                        })
                        .or_insert(value);
                }
            }
        }

        if multiply {
            dec!(1)
                - (values
                    .into_values()
                    .map(|val| dec!(1) - val)
                    .product::<Decimal>()
                    * stacking)
        } else {
            stacking + values.into_values().sum::<Decimal>()
        }
    }
}
