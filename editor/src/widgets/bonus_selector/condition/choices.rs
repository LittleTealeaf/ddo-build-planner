use core::fmt::{self, Display};

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
