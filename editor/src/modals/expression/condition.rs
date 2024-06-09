use core::fmt::{self, Display};

use crate::modals::expression::ModalExpressionType;
use builder::bonus::Condition;

use super::{InternalSelector, ModalExpression};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionType {
    Not,
    GreaterThan,
    LessThan,
    EqualTo,
    True,
    False,
    And,
    Or,
    Xor,
}

impl ConditionType {
    pub const TYPES: [Self; 9] = [
        Self::Not,
        Self::GreaterThan,
        Self::LessThan,
        Self::EqualTo,
        Self::True,
        Self::False,
        Self::And,
        Self::Or,
        Self::Xor,
    ];
}

impl Display for ConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => write!(f, "Not"),
            Self::GreaterThan => write!(f, "Greater Than"),
            Self::LessThan => write!(f, "Less Than"),
            Self::EqualTo => write!(f, "Equal To"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "Fase"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Xor => write!(f, "Xor"),
        }
    }
}

impl ModalExpression {
    pub(super) fn get_internal_condition(&self, id: usize) -> Option<Condition> {
        let selector = self.selectors.get(&id)?;
        let ModalExpressionType::Condition(condition_type) = selector.selector_type else {
            return None;
        };

        let val_a = selector.value_a.and_then(|id| self.get_internal_value(id));
        let val_b = selector.value_b.and_then(|id| self.get_internal_value(id));
        let cond_a = selector
            .condition_a
            .and_then(|id| self.get_internal_condition(id));
        let cond_b = selector
            .condition_b
            .and_then(|id| self.get_internal_condition(id));

        Some(match condition_type {
            ConditionType::Not => !cond_a?,
            ConditionType::GreaterThan => val_a?.greater_than(val_b?),
            ConditionType::LessThan => val_a?.less_than(val_b?),
            ConditionType::EqualTo => val_a?.equal_to(val_b?),
            ConditionType::True => Condition::TRUE,
            ConditionType::False => Condition::FALSE,
            ConditionType::And => cond_a? & cond_b?,
            ConditionType::Or => cond_a? | cond_b?,
            ConditionType::Xor => cond_a? ^ cond_b?,
        })
    }
}

impl InternalSelector {
    pub fn condition(selector: &mut ModalExpression, condition: Option<Condition>) -> Self {
        let Some(condition) = condition else {
            return Self::condition(selector, Some(Condition::TRUE));
        };

        let mut value_a = None;
        let mut value_b = None;
        let mut condition_a = None;
        let mut condition_b = None;

        let mut condition_type = ConditionType::True;

        match condition {
            Condition::Not(condition) => {
                condition_a = Some(selector.add_selector_condition(*condition));
                condition_type = ConditionType::Not;
            }
            Condition::GreaterThan(a, b) => {
                value_a = Some(selector.add_selector_value(a));
                value_b = Some(selector.add_selector_value(b));
                condition_type = ConditionType::GreaterThan;
            }
            Condition::LessThan(a, b) => {
                value_a = Some(selector.add_selector_value(a));
                value_b = Some(selector.add_selector_value(b));
                condition_type = ConditionType::LessThan;
            }
            Condition::EqualTo(a, b) => {
                value_a = Some(selector.add_selector_value(a));
                value_b = Some(selector.add_selector_value(b));
                condition_type = ConditionType::EqualTo;
            }
            Condition::Constant(value) => {
                if !value {
                    condition_type = ConditionType::False;
                }
            }
            Condition::And(a, b) => {
                condition_a = Some(selector.add_selector_condition(*a));
                condition_b = Some(selector.add_selector_condition(*b));
                condition_type = ConditionType::And;
            }
            Condition::Or(a, b) => {
                condition_a = Some(selector.add_selector_condition(*a));
                condition_b = Some(selector.add_selector_condition(*b));
                condition_type = ConditionType::Or;
            }
            Condition::Xor(a, b) => {
                condition_a = Some(selector.add_selector_condition(*a));
                condition_b = Some(selector.add_selector_condition(*b));
                condition_type = ConditionType::Xor;
            }
        }

        Self {
            value_a,
            value_b,
            condition_a,
            condition_b,
            attribute: None,
            constant: None,
            constant_str: String::new(),
            selector_type: ModalExpressionType::Condition(condition_type),
        }
    }
}
