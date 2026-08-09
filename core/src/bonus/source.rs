use crate::attribute::Attribute;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BonusSource {
    #[cfg(feature="debug")]
    Debug(u8),
    #[display("{_0}")]
    Custom(String),
    #[display("Attribute: {_0}")]
    Attribute(Attribute),
    #[display("Item: {_0}")]
    Item(String),
    #[display("Set Bonus: {_0}")]
    SetBonus(String),
    #[display("Feat: {_0}")]
    Feat(String)
}

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}
