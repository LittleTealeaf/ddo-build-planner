use std::fmt::Display;

use enum_map::Enum;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum WeaponHand {
    Main,
    Off,
}

impl WeaponHand {
    const BOTH: [Self; 2] = [Self::Main, Self::Off];
}

impl Display for WeaponHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponHand::Main => write!(f, "Main"),
            WeaponHand::Off => write!(f, "Off"),
        }
    }
}
