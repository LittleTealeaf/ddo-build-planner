use core::fmt;

use fmt::Display;

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, ToAttribute},
    types::item_type::{ShieldType, WeaponType},
};

use super::{Flag, ToFlag};

/// Represents the different types of items the character can wield in their off hand
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OffHandType {
    /// The character is wielding a weapon in their off hand
    #[serde(rename = "w", alias = "Weapon")]
    Weapon(WeaponType),
    /// The character is wielding a shield in their off-hand
    #[serde(rename = "s", alias = "Shield")]
    Shield(ShieldType),
    /// The character is wielding a runearm in their off-hand
    #[serde(rename = "r", alias = "RuneArm")]
    RuneArm,
}

impl Display for OffHandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weapon(weapon) => weapon.fmt(f),
            Self::Shield(shield) => shield.fmt(f),
            Self::RuneArm => write!(f, "Rune Arm"),
        }
    }
}

impl From<WeaponType> for OffHandType {
    fn from(value: WeaponType) -> Self {
        Self::Weapon(value)
    }
}

impl From<ShieldType> for OffHandType {
    fn from(value: ShieldType) -> Self {
        Self::Shield(value)
    }
}

impl ToAttribute for OffHandType {
    fn to_attribute(self) -> Attribute {
        self.to_flag().to_attribute()
    }
}

impl ToFlag for OffHandType {
    fn to_flag(self) -> Flag {
        Flag::OffHandType(self)
    }
}

impl StaticValues for OffHandType {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            WeaponType::values().map(Self::Weapon),
            ShieldType::values().map(Self::Shield),
            [Self::RuneArm]
        )
    }
}
