#[derive(Debug, Clone, Eq, PartialEq, Copy, Default)]
pub enum ValueChoice {
    #[default]
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

impl ValueChoice {
    pub const ALL: [Self; 15] = [
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
}
