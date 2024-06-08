use builder::bonus::Value;

use super::{InternalSelector, ModalExpression, ModalExpressionType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Const,
    Attribute,
    Min,
    Max,
    Floor,
    Ceil,
    Round,
    Abs,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    If,
    Dice,
}

impl ModalExpression {
    pub(super) fn get_internal_value(&self, id: usize) -> Option<Value> {
        let selector = self.selectors.get(&id)?;
        let ModalExpressionType::Value(value_type) = selector.selector_type else {
            return None;
        };

        let val_a = selector.value_a.and_then(|id| self.get_internal_value(id));
        let val_b = selector.value_b.and_then(|id| self.get_internal_value(id));
        let cond_a = selector
            .condition_a
            .and_then(|id| self.get_internal_condition(id));

        Some(match value_type {
            ValueType::Const => Value::Const(selector.constant?),
            ValueType::Attribute => Value::Attribute(selector.attribute.clone()?),
            ValueType::Min => val_a?.min(val_b?),
            ValueType::Max => val_a?.max(val_b?),
            ValueType::Floor => val_a?.floor(),
            ValueType::Ceil => val_a?.ceil(),
            ValueType::Round => val_a?.round(),
            ValueType::Abs => val_a?.abs(),
            ValueType::Add => val_a? + val_b?,
            ValueType::Sub => val_a? - val_b?,
            ValueType::Mul => val_a? * val_b?,
            ValueType::Div => val_a? / val_b?,
            ValueType::Rem => val_a? % val_b?,
            ValueType::If => Value::condition(cond_a?, val_a?, val_b?),
            ValueType::Dice => Value::dice(val_a?, val_b?),
        })
    }
}

impl InternalSelector {
    pub fn value(selector: &mut ModalExpression, value: Option<Value>) -> Self {
        let Some(value) = value else {
            return Self::value(selector, Some(Value::ONE));
        };

        let mut value_a = None;
        let mut value_b = None;
        let mut condition_a = None;
        let mut constant = None;
        let mut attribute = None;

        let mut value_type = ValueType::Const;

        match value {
            Value::Const(value) => {
                constant = Some(value);
            }
            Value::Attribute(attr) => {
                attribute = Some(attr);
                value_type = ValueType::Attribute;
            }
            Value::Min(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Min;
            }
            Value::Max(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Max;
            }
            Value::Floor(value) => {
                value_a = Some(selector.add_selector_value(*value));
                value_type = ValueType::Floor;
            }
            Value::Ceil(value) => {
                value_a = Some(selector.add_selector_value(*value));
                value_type = ValueType::Ceil;
            }
            Value::Round(value) => {
                value_a = Some(selector.add_selector_value(*value));
                value_type = ValueType::Round;
            }
            Value::Abs(value) => {
                value_a = Some(selector.add_selector_value(*value));
                value_type = ValueType::Abs;
            }
            Value::Add(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Add;
            }
            Value::Sub(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Sub;
            }
            Value::Mul(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Mul;
            }
            Value::Div(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Div;
            }
            Value::Rem(a, b) => {
                value_a = Some(selector.add_selector_value(*a));
                value_b = Some(selector.add_selector_value(*b));
                value_type = ValueType::Rem;
            }
            Value::If {
                condition,
                if_true,
                if_false,
            } => {
                condition_a = Some(selector.add_selector_condition(*condition));
                value_a = Some(selector.add_selector_value(*if_true));
                value_b = Some(selector.add_selector_value(*if_false));
                value_type = ValueType::If;
            }
            Value::Dice { count, size } => {
                value_a = Some(selector.add_selector_value(*count));
                value_b = Some(selector.add_selector_value(*size));
                value_type = ValueType::Dice;
            }
        }

        Self {
            value_a,
            value_b,
            condition_a,
            condition_b: None,
            attribute,
            constant,
            constant_str: constant.map(|val| val.to_string()).unwrap_or_default(),
            selector_type: ModalExpressionType::Value(value_type),
        }
    }
}
