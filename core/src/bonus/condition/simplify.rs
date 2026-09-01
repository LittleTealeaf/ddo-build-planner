use core::ops::Not;

use crate::bonus::BonusCondition;

impl BonusCondition {
    #[must_use]
    pub fn simplify(self) -> Self {
        match self {
            Self::And(left, right) => {
                let mut conditions = Vec::new();
                let mut stack = vec![*left, *right];
                while let Some(item) = stack.pop() {
                    match item {
                        Self::And(left, right) => {
                            stack.push(*left);
                            stack.push(*right);
                        }
                        Self::Constant(true) => {}
                        Self::Constant(false) => {
                            return Self::FALSE;
                        }
                        other => conditions.push(other.simplify()),
                    }
                }
                Self::all(conditions)
            }
            Self::Or(left, right) => {
                let mut conditions = Vec::new();
                let mut stack = vec![*left, *right];
                while let Some(item) = stack.pop() {
                    match item {
                        Self::And(left, right) => {
                            stack.push(*left);
                            stack.push(*right);
                        }
                        Self::Constant(true) => {
                            return Self::TRUE;
                        }
                        Self::Constant(false) => {}
                        other => conditions.push(other.simplify()),
                    }
                }
                Self::any(conditions)
            }
            Self::Not(condition) => match *condition {
                Self::Not(value) => *value,
                other => other.not(),
            },
            other => other,
        }
    }
}
