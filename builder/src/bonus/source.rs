use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{attribute::Attribute, equipment::ItemSlot};

/// Dictates the source that a bonus comes from.
///
/// Each bonus must have a source that dictates where that bonus came from. For example, if an attribute returns any new bonuses, they must all have a source of that attribute.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BonusSource {
    /// Indicates that the bonus comes from an attribute.
    Attribute(Attribute),
    /// Indicates that the bonus comes from a particular item slot
    ItemSlot(ItemSlot),
    /// Dictates any custom bonuses for general uses. When possible, do not use this source
    Custom(u8),
    /// Used for debugging purposes.
    #[cfg(test)]
    Debug(u8),
    /// Only used for initial values
    Base,
}

impl Display for BonusSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BonusSource::Attribute(attr) => write!(f, "Attribute: {}", attr),
            BonusSource::Custom(num) => write!(f, "Custom: {}", num),
            #[cfg(test)]
            BonusSource::Debug(num) => write!(f, "Debug: {}", num),
            BonusSource::Base => write!(f, "Base"),
            BonusSource::ItemSlot(slot) => write!(f, "{}", slot),
        }
    }
}

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}

impl From<u8> for BonusSource {
    fn from(value: u8) -> Self {
        Self::Custom(value)
    }
}

impl From<ItemSlot> for BonusSource {
    fn from(value: ItemSlot) -> Self {
        Self::ItemSlot(value)
    }
}
