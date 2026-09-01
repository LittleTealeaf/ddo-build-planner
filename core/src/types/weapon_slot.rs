#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    strum::VariantArray,
)]
pub enum WeaponSlot {
    #[display("Main Hand")]
    MainHand,
    #[display("Off Hand")]
    OffHand,
}
