


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
