use serde::{Serialize, Deserialize};




/// Dinosuar Bone Crafting from the Isle of Dread expansion
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Hash)]
pub enum DinoBoneSlot {
    /// Scale Slot (Armor)
    ArmorScale,
    /// Fang Slot (Armor)
    ArmorFang,
    /// Scale Slot (Weapon)
    WeaponScale,
    /// Fang Slot (Weapon)
    WeaponFang,
    /// Claw Slot (Weapon)
    WeaponClaw,
    /// Horn Slot (Weapon)
    WeaponHorn,
    /// Scale Slot (Accessory)
    AccessoryScale,
    /// Fang Slot (Accessory)
    AccessoryFang,
    /// Claw Slot (Accessory)
    AccessoryClaw,
    /// Horn Slot (Accessory)
    AccessoryHorn,
    /// Set Bonus
    SetBonus
}
