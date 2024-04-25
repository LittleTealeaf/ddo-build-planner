use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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

impl ValueType {
    pub const TYPES: [Self; 15] = [
        Self::Const,
        Self::Attribute,
        Self::Min,
        Self::Max,
        Self::Floor,
        Self::Ceil,
        Self::Round,
        Self::Abs,
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Rem,
        Self::If,
        Self::Dice,
    ];

    pub const fn show_value_a(self) -> bool {
        !matches!(self, Self::Const | Self::Attribute)
    }

    pub const fn show_value_b(self) -> bool {
        matches!(
            self,
            Self::Min
                | Self::Max
                | Self::Add
                | Self::Sub
                | Self::Mul
                | Self::Div
                | Self::Rem
                | Self::If
                | Self::Dice
        )
    }

    pub const fn show_attribute(self) -> bool {
        matches!(self, Self::Attribute)
    }

    pub const fn show_condition(self) -> bool {
        matches!(self, Self::If)
    }

    pub const fn show_const(self) -> bool {
        matches!(self, Self::Const)
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const => write!(f, "Constant"),
            Self::Attribute => write!(f, "Attribute"),
            Self::Min => write!(f, "Min"),
            Self::Max => write!(f, "Max"),
            Self::Floor => write!(f, "Floor"),
            Self::Ceil => write!(f, "Ceil"),
            Self::Round => write!(f, "Round"),
            Self::Abs => write!(f, "Absolute"),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Subtract"),
            Self::Mul => write!(f, "Multiply"),
            Self::Rem => write!(f, "Remainder"),
            Self::If => write!(f, "If"),
            Self::Dice => write!(f, "Dice"),
            Self::Div => write!(f, "Divide"),
        }
    }
}
