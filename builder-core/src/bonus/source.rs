use crate::attribute::Attribute;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BonusSource {
    Attribute(Attribute),
    Custom(u8),
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
