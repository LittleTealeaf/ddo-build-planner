use crate::types::equipment_slot::EquipmentSlot;

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
    #[cfg(feature = "debug")]
    Debug(String),
    Core,
    Feats,
    SetBonus(String),
    #[display("{_0} (Set {_1})")]
    Equipment(EquipmentSlot, u32),
}
