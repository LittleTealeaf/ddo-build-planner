use enum_map::Enum;

use crate::attribute::Attribute;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
pub enum BonusSource {
    Attribute(Attribute),
    /// Only for debug(?)
    Custom(u8),
    /// Only for debug
    #[cfg(test)]
    Test(u8),
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
