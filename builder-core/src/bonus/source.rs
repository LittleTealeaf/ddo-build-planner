use enum_map::Enum;

use crate::attribute::Attribute;

/// Dictates the source that a bonus comes from.
///
/// Each bonus must have a source that dictates where that bonus came from. For example, if an attribute returns any new bonuses, they must all have a source of that attribute.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
pub enum BonusSource {
    /// Indicates that the bonus comes from an attribute.
    Attribute(Attribute),
    /// Dictates any custom bonuses for general uses. When possible, do not use this source
    Custom(u8),
    /// Used for debugging purposes.
    #[cfg(test)]
    Debug(u8),
    /// Only used for initial values
    Base,
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
