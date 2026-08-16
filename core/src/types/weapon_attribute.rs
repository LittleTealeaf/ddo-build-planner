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
pub enum WeaponAttribute {
    Attack,
    Damage,
    CriticalThreat,
}

impl WeaponAttribute {
    pub const VALUES: [Self; 3] = [Self::Attack, Self::Damage, Self::CriticalThreat];
}

impl IterValues for WeaponAttribute {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
