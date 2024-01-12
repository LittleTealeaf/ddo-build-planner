use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::attribute::{Attribute, ToAttribute};

/// Dictates the source that a bonus comes from.
///
/// Each bonus must have a source that dictates where that bonus came from. For example, if an attribute returns any new bonuses, they must all have a source of that attribute.
#[derive(Hash, Clone, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BonusSource {
    /// Indicates that the bonus comes from an attribute.
    Attribute(Attribute),
    /// Dictates any custom bonuses for general uses. When possible, do not use this source
    Custom(u8),
    /// Used for debugging purposes.
    #[cfg(feature = "debug")]
    Debug(u8),
    /// Only used for initial values
    Base,
    /// Only used for the Set Bonus importing
    SetBonus
}

impl Display for BonusSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attribute(attr) => write!(f, "Attribute: {attr}"),
            Self::Custom(num) => write!(f, "Custom: {num}"),
            #[cfg(feature = "debug")]
            Self::Debug(num) => write!(f, "Debug: {num}"),
            Self::Base => write!(f, "Base"),
            Self::SetBonus => write!(f, "Set Bonus"),
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

impl<T> From<T> for BonusSource
where
    T: ToAttribute,
{
    fn from(value: T) -> Self {
        value.to_attribute().into()
    }
}
