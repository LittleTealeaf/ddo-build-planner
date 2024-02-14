use core::fmt::{self, Display};

use builder::bonus::{Condition, Value};

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy)]
pub enum ConditionChoice {
    Not,
    GreaterThan,
    LessThan,
    EqualTo,
    #[default]
    True,
    False,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Field {
    ValueA,
    ValueB,
    ConditionA,
    ConditionB,
}

impl ConditionChoice {
    pub const ALL: [Self; 9] = [
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

    pub fn get_fields(self) -> Vec<Field> {
        match self {
            Self::Not => vec![Field::ConditionA],
            Self::GreaterThan | Self::LessThan | Self::EqualTo => {
                vec![Field::ValueA, Field::ValueB]
            }
            Self::True | Self::False => Vec::new(),
            Self::And | Self::Or | Self::Xor => vec![Field::ConditionA, Field::ConditionB],
        }
    }

    pub fn display(
        self,
        value_a: &Option<Value>,
        value_b: &Option<Value>,
        condition_a: &Option<Condition>,
        condition_b: &Option<Condition>,
    ) -> String {
        let val_a = value_a
            .as_ref()
            .map_or("[Val A]".to_owned(), |val| format!("{val}"));

        let val_b = value_b
            .as_ref()
            .map_or("[Val B]".to_owned(), |val| format!("{val}"));

        let cond_a = condition_a
            .as_ref()
            .map_or("[Cond A]".to_owned(), |val| format!("{val}"));
        let cond_b = condition_b
            .as_ref()
            .map_or("[Cond B]".to_owned(), |val| format!("{val}"));

        match self {
            Self::Not => format!("Not {cond_a}"),
            Self::GreaterThan => format!("{val_a} > {val_b}"),
            Self::LessThan => format!("{val_a} < {val_b}"),
            Self::EqualTo => format!("{val_a} == {val_b}"),
            Self::True => "True".to_owned(),
            Self::False => "False".to_owned(),
            Self::And => format!("{cond_a} && {cond_b}"),
            Self::Or => format!("{cond_a} || {cond_b}"),
            Self::Xor => format!("{cond_a} ^ {cond_b}"),
        }
    }
}

impl Display for ConditionChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Not => write!(f, "Not"),
            Self::GreaterThan => write!(f, "Greater Than"),
            Self::LessThan => write!(f, "Less Than"),
            Self::EqualTo => write!(f, "Equal To"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Xor => write!(f, "Xor"),
        }
    }
}
