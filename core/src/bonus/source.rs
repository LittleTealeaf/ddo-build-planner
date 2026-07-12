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
    #[display("Attribute: {_0}")]
    Attribute(Attribute),
    #[display("Item: {_0}")]
    Item(String),
    #[display("Set Bonus: {_0}")]
    SetBonus(String),
}

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        Self::Attribute(value)
    }
}
