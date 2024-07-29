//! User interactable sliders in the builder

use core::fmt;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, BonusSource, BonusType, Value},
};

use super::flag::{Flag, ToFlag};

/// Indicates sliders that the user is able to interact with
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Slider {
    /// Deific Warding Stacks
    #[serde(rename = "dw", alias = "DeificWarding")]
    DeificWarding,
    /// Ascendency
    #[serde(rename = "as", alias = "Ascendency")]
    Ascendency,
    /// Archer's Focus
    #[serde(rename = "af", alias = "ArchersFocus")]
    ArchersFocus,
    /// Source from the Angel of Vengeance tree
    #[serde(rename = "avs", alias = "Scourge")]
    Scourge,
    /// Optimism from the Beacon of Hope tree
    #[serde(rename = "bho", alias = "Optimism")]
    Optimism,
    /// Erosion from the Earth Savant tree
    #[serde(rename = "ese", alias = "Erosion")]
    Erosion,
    /// Conflagration from the Fire Savant tree
    #[serde(rename = "fsc", alias = "Conflagration")]
    Conflagration,
    /// Hoarfrost from the Water Savant tree
    #[serde(rename = "wsh", alias = "Hoarfrost")]
    Hoarfrost,
    /// Alternating Current from the Air Savant tree
    #[serde(rename = "asa", alias = "AlternatingCurrent")]
    AlternatingCurrent,
    /// Weapon Bond from the Occult Slayer tree
    #[serde(rename = "osw", alias = "WeaponBond")]
    WeaponBond,
}

impl Slider {
    /// Returns the base stack size of the slider
    #[must_use]
    pub const fn base_stack_max(&self) -> Decimal {
        match self {
            Self::Erosion | Self::Conflagration | Self::Hoarfrost | Self::AlternatingCurrent => {
                dec!(1)
            }
            Self::Scourge => dec!(3),
            Self::DeificWarding | Self::Optimism => dec!(10),
            Self::ArchersFocus | Self::Ascendency => dec!(15),
            Self::WeaponBond => dec!(200),
        }
    }

    /// Creates a new bonus that sets the value of the slider
    #[must_use]
    pub fn slider_bonus<V>(&self, value: V) -> Bonus
    where
        V: Into<Value>,
    {
        Bonus::new(
            self.to_attribute(),
            BonusType::Stacking,
            value,
            BonusSource::Slider(*self),
        )
    }
}

impl fmt::Display for Slider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeificWarding => write!(f, "Deific Warding"),
            Self::Ascendency => write!(f, "Ascendency"),
            Self::ArchersFocus => write!(f, "Archer's Focus"),
            Self::Scourge => write!(f, "Scourge"),
            Self::Optimism => write!(f, "Optimism"),
            Self::Erosion => write!(f, "Erosion"),
            Self::Conflagration => write!(f, "Conflagration"),
            Self::Hoarfrost => write!(f, "Hoarfrost"),
            Self::AlternatingCurrent => write!(f, "Alternating Current"),
            Self::WeaponBond => write!(f, "Weapon Bond"),
        }
    }
}

impl ToFlag for Slider {
    fn to_flag(self) -> Flag {
        Flag::HasSlider(self)
    }
}

impl ToAttribute for Slider {
    fn to_attribute(self) -> Attribute {
        Attribute::Slider(self)
    }
}

/// Indicates that a type can be directly converted to a slider
pub trait ToSlider {
    /// Converts to a slider
    fn to_slider(self) -> Slider;
}

impl<T> From<T> for Slider
where
    T: ToSlider,
{
    fn from(value: T) -> Self {
        value.to_slider()
    }
}

impl StaticValues for Slider {
    fn values() -> impl Iterator<Item = Self> {
        [
            Self::DeificWarding,
            Self::Ascendency,
            Self::ArchersFocus,
            Self::Scourge,
            Self::Optimism,
            Self::Conflagration,
            Self::Erosion,
            Self::Hoarfrost,
            Self::AlternatingCurrent,
            Self::WeaponBond,
        ]
        .into_iter()
    }
}
