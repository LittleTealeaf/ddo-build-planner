use crate::traits::IterValues;

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
)]
pub enum WeaponSlot {
    #[display("Main Hand")]
    MainHand,
    #[display("Off Hand")]
    OffHand,
}

impl WeaponSlot {
    pub const VALUES: [Self; 2] = [Self::MainHand, Self::OffHand];
}

impl IterValues for WeaponSlot {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
