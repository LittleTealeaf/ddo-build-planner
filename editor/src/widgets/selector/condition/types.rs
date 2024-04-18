use core::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

impl ConditionType {
    pub const fn show_condition_a(self) -> bool {
        matches!(self, Self::Not | Self::And | Self::Or | Self::Xor)
    }

    pub const fn show_condition_b(self) -> bool {
        matches!(self, Self::And | Self::Or | Self::Xor)
    }

    pub const fn show_value_a(self) -> bool {
        matches!(self, Self::GreaterThan | Self::LessThan | Self::EqualTo)
    }

    pub const fn show_value_b(self) -> bool {
        matches!(self, Self::GreaterThan | Self::LessThan | Self::EqualTo)
    }
}

impl Display for ConditionType {
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
