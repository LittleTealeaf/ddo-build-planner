use std::fmt::Display;

use enum_map::Enum;

use super::toggles::Toggle;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
pub enum Flag {
    HasToggle(Toggle),
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::HasToggle(toggle) => write!(f, "Has {} Toggle", toggle),
        }
    }
}
