use std::fmt::Display;

use enum_map::Enum;

use crate::attribute::{flags::Flag, Attribute};



#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum Immunity {
    Sleep,
}

impl Display for Immunity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immunity::Sleep => write!(f, "Sleep"),
        }
    }
}

impl From<Immunity> for Attribute {
    fn from(value: Immunity) -> Self {
        Flag::from(value).into()
    }
}
