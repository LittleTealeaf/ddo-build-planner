use std::fmt::Display;

use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Alignment {
    Good,
    LawfulGood,
    ChaoticGood,
    Evil,
    LawfulEvil,
    ChaoticEvil,
    Neutral,
    LawfulNeutral,
    ChaoticNeutral
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Good => write!(f, "Good"),
            Alignment::LawfulGood => write!(f, "Lawful Good"),
            Alignment::ChaoticGood => write!(f, "Chaotic Good"),
            Alignment::Evil => write!(f, "Evil"),
            Alignment::LawfulEvil => write!(f, "Lawful Evil"),
            Alignment::ChaoticEvil => write!(f, "Chaotic Evil"),
            Alignment::Neutral => write!(f, "Neutral"),
            Alignment::LawfulNeutral => write!(f, "Lawful Neutral"),
            Alignment::ChaoticNeutral => write!(f, "Chaotic Neutral"),
        }
    }
}
