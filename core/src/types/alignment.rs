use crate::traits::IterValues;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum Alignment {
    Good,
    Evil,
    Neutral,
    Lawful,
    Chaotic,
}

impl Alignment {
    pub const VALUES: [Self; 5] = [
        Self::Good,
        Self::Evil,
        Self::Neutral,
        Self::Lawful,
        Self::Chaotic,
    ];
}

impl IterValues for Alignment {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
