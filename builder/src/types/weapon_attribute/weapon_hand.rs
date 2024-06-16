use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// The hand used for a Weapon Attribute
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponHand {
    /// Both Hands
    #[serde(rename = "b", alias = "Both")]
    Both,
    /// The Main Hand
    #[serde(rename = "m", alias = "Main")]
    Main,
    /// The Off Hand
    #[serde(rename = "o", alias = "Off")]
    Off,
}

impl WeaponHand {
    /// Both the Main Hand and the Off Hand
    pub const HANDS: [Self; 2] = [Self::Main, Self::Off];
}

impl Display for WeaponHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Main => write!(f, "Main"),
            Self::Off => write!(f, "Off"),
            Self::Both => write!(f, "Both"),
        }
    }
}

impl StaticValues for WeaponHand {
    fn values() -> impl Iterator<Item = Self> {
        [Self::Both, Self::Main, Self::Off].into_iter()
    }
}
