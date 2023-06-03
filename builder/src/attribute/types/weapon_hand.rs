use std::fmt::Display;

use enum_map::Enum;

/// The hand used for a Weapon Attribute
#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum WeaponHand {
    /// The Main Hand
    Main,
    /// The Off Hand
    Off,
    /// Both Hands
    Both,
}

impl WeaponHand {
    /// Both the Main Hand and the Off Hand
    pub const VALUES: [Self; 2] = [Self::Main, Self::Off];
}

impl Display for WeaponHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponHand::Main => write!(f, "Main"),
            WeaponHand::Off => write!(f, "Off"),
            WeaponHand::Both => write!(f, "Both"),
        }
    }
}
