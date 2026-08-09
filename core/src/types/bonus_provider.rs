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
    Item(String),
    SetBonus(String),
    Equipment(EquipmentSlot),
}
