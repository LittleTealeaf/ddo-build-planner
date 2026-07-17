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
    Item(String),
    SetBonus(String),
}
