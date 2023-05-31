use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, CloneBonus},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Sheltering {
    Physical,
    Magical,
    MagicalCap,
    Both,
}

impl CloneBonus for Sheltering {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::Both).then(|| {
            [Self::Physical, Self::Magical]
                .map(|sheltering| {
                    Bonus::new(
                        sheltering.into(),
                        bonus.get_type(),
                        bonus.get_value(),
                        bonus.get_source(),
                        bonus.get_condition(),
                    )
                })
                .to_vec()
        })
    }
}

impl Display for Sheltering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sheltering::Physical => write!(f, "Physical Sheltering"),
            Sheltering::Magical => write!(f, "Magical Sheltering"),
            Sheltering::MagicalCap => write!(f, "Magical Sheltering Cap"),
            Sheltering::Both => write!(f, "Sheltering"),
        }
    }
}

impl From<Sheltering> for Attribute {
    fn from(value: Sheltering) -> Self {
        Self::Sheltering(value)
    }
}