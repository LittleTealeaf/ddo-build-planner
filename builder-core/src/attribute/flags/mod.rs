use std::fmt::Display;

use enum_map::Enum;

use super::toggles::Toggle;

/// Indicates that the character possesses some flag.
///
/// Flags are most often used for indirect effects, such as "does the character have this toggle", or other traits.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
pub enum Flag {
    /// Indicates that the user has access to a given toggle.
    HasToggle(Toggle),
}

impl Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flag::HasToggle(toggle) => write!(f, "Has {} Toggle", toggle),
        }
    }
}
