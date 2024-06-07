use builder::bonus::{Condition, Value};

use self::types::ConditionType;

use super::{value::ValueSelector, SelectorWidgetMessage};

pub mod message;
pub mod types;
pub mod view;

#[derive(Debug, Clone)]
pub struct ConditionSelector {
    depth: usize,
    selector: Option<ConditionSubSelector>,
    on_submit: SelectorWidgetMessage,
    on_cancel: SelectorWidgetMessage,
    cond: ConditionType,
    condition_a: Option<Condition>,
    condition_b: Option<Condition>,
    value_a: Option<Value>,
    value_b: Option<Value>,
}

impl ConditionSelector {
    pub fn new<'a, V>(
        depth: usize,
        value: V,
        on_submit: SelectorWidgetMessage,
        on_cancel: SelectorWidgetMessage,
    ) -> Self
    where
        V: Into<Option<&'a Condition>>,
    {
        let value = value.into();

        let mut value_a = None;
        let mut value_b = None;
        let mut condition_a = None;
        let mut condition_b = None;

        let cond = match value.unwrap_or(&Condition::TRUE) {
            Condition::Not(cond) => {
                condition_a = Some(*cond.clone());
                ConditionType::Not
            }
            Condition::GreaterThan(a, b) => {
                value_a = Some(a.clone());
                value_b = Some(b.clone());
                ConditionType::GreaterThan
            }
            Condition::LessThan(a, b) => {
                value_a = Some(a.clone());
                value_b = Some(b.clone());
                ConditionType::LessThan
            }
            Condition::EqualTo(a, b) => {
                value_a = Some(a.clone());
                value_b = Some(b.clone());
                ConditionType::EqualTo
            }
            Condition::Constant(value) => {
                if *value {
                    ConditionType::True
                } else {
                    ConditionType::False
                }
            }
            Condition::And(a, b) => {
                condition_a = Some(*a.clone());
                condition_b = Some(*b.clone());
                ConditionType::And
            }
            Condition::Or(a, b) => {
                condition_a = Some(*a.clone());
                condition_b = Some(*b.clone());
                ConditionType::Or
            }
            Condition::Xor(a, b) => {
                condition_a = Some(*a.clone());
                condition_b = Some(*b.clone());
                ConditionType::Xor
            }
        };

        let selector = None;

        Self {
            depth,
            selector,
            on_submit,
            on_cancel,
            cond,
            condition_a,
            condition_b,
            value_a,
            value_b,
        }
    }

    pub fn get_condition(&self) -> Option<Condition> {
        Some(match self.cond {
            ConditionType::Not => !self.condition_a.clone()?,
            ConditionType::GreaterThan => self.value_a.clone()?.greater_than(self.value_b.clone()?),
            ConditionType::LessThan => self.value_a.clone()?.less_than(self.value_b.clone()?),
            ConditionType::EqualTo => self.value_a.clone()?.equal_to(self.value_b.clone()?),
            ConditionType::True => Condition::TRUE,
            ConditionType::False => Condition::FALSE,
            ConditionType::And => self.condition_a.clone()? & self.condition_b.clone()?,
            ConditionType::Or => self.condition_a.clone()? | self.condition_b.clone()?,
            ConditionType::Xor => self.condition_a.clone()? ^ self.condition_b.clone()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ConditionSubSelector {
    ConditionA(Box<ConditionSelector>),
    ConditionB(Box<ConditionSelector>),
    ValueA(Box<ValueSelector>),
    ValueB(Box<ValueSelector>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_condition_type() {
        let tests = [
            (ConditionType::Not, !Condition::TRUE),
            (
                ConditionType::GreaterThan,
                Value::ONE.greater_than(Value::TWO),
            ),
            (ConditionType::LessThan, Value::ONE.less_than(Value::TWO)),
            (ConditionType::EqualTo, Value::ONE.equal_to(Value::TWO)),
            (ConditionType::True, Condition::TRUE),
            (ConditionType::False, Condition::FALSE),
            (ConditionType::And, Condition::TRUE & Condition::FALSE),
            (ConditionType::Or, Condition::TRUE | Condition::FALSE),
            (ConditionType::Xor, Condition::TRUE ^ Condition::FALSE),
        ];

        for (cond_type, condition) in tests {
            let selector = ConditionSelector::new(
                0,
                &condition,
                SelectorWidgetMessage::Submit,
                SelectorWidgetMessage::Submit,
            );

            assert_eq!(selector.cond, cond_type);
        }
    }
}
