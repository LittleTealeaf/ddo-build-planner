use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::all::AllStatic;

/// The hand used for a Weapon Attribute
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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

impl AllStatic for WeaponHand {
    fn all() -> impl Iterator<Item = Self> {
        [
            Self::Both,
            Self::Main,
            Self::Off
        ].into_iter()
    }
}
