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

    pub fn displayables(self) -> (&'static str, Vec<(&'static str, Field)>) {
        match self {
            Self::Not => ("Not [A]", vec![("A", Field::ConditionA)]),
            Self::GreaterThan => (
                "[A] > [B]",
                vec![("A", Field::ValueA), ("B", Field::ValueB)],
            ),
            Self::LessThan => (
                "[A] < [B]",
                vec![("A", Field::ValueA), ("B", Field::ValueB)],
            ),
            Self::EqualTo => (
                "[A] == [B]",
                vec![("A", Field::ValueA), ("B", Field::ValueB)],
            ),
            Self::True => ("True", Vec::new()),
            Self::False => ("False", Vec::new()),
            Self::And => (
                "[A] && [B]",
                vec![("A", Field::ConditionA), ("B", Field::ConditionB)],
            ),
            Self::Or => (
                "[A] || [B]",
                vec![("A", Field::ConditionA), ("B", Field::ConditionB)],
            ),
            Self::Xor => (
                "[A] ^ [B]",
                vec![("A", Field::ConditionA), ("B", Field::ConditionB)],
            ),
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
