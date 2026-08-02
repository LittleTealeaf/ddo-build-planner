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
pub enum BonusProvider {
    Core,
    #[cfg(feature = "debug")]
    Debug(String),
    Item(String),
    SetBonus(String),
}
