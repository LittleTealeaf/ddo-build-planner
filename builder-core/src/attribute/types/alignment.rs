use std::fmt::Display;

use enum_map::Enum;

/// Determines alignment. To create a complete alignment, two of these attributes are required.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Alignment {
    /// Good
    Good,
    /// Evil
    Evil,
    /// Neutral
    Neutral,
    /// Lawful
    Lawful,
    /// Chaotic
    Chaotic,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Good => write!(f, "Good"),
            Alignment::Evil => write!(f, "Evil"),
            Alignment::Neutral => write!(f, "Neutral"),
            Alignment::Lawful => write!(f, "Lawful"),
            Alignment::Chaotic => write!(f, "Chaotic"),
        }
    }
}