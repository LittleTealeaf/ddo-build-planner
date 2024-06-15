use core::fmt;

use fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, ToAttribute},
    types::item_type::WeaponType,
};

use super::{Flag, ToFlag};

/// Represents the different options that the character can have in their main hand.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MainHandType {
    /// The character is wielding a wand in their main hand.
    Wand,
    /// The character is wielding some weapon in their main hand.
    Weapon(WeaponType),
}

impl Display for MainHandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wand => write!(f, "Wand"),
            Self::Weapon(weapon) => weapon.fmt(f),
        }
    }
}

impl From<WeaponType> for MainHandType {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}

impl ToFlag for MainHandType {
    fn to_flag(self) -> Flag {
        Flag::MainHandType(self)
    }
}

impl ToAttribute for MainHandType {
    fn to_attribute(self) -> Attribute {
        self.to_flag().to_attribute()
    }
}

impl StaticValues for MainHandType {
    fn values() -> impl Iterator<Item = Self> {
        chain!([Self::Wand], WeaponType::values().map(Self::Weapon))
    }
}
