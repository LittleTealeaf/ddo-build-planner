use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The hand used for a Weapon Attribute
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponHand {
    /// Both Hands
    Both,
    /// The Main Hand
    Main,
    /// The Off Hand
    Off,
}

impl WeaponHand {
    /// Both the Main Hand and the Off Hand
    pub const VALUES: [Self; 2] = [Self::Main, Self::Off];
}

impl Display for WeaponHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => write!(f, "Main"),
            Self::Off => write!(f, "Off"),
            Self::Both => write!(f, "Both"),
        }
    }
}
