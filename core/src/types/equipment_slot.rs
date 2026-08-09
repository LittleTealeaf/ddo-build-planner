use itertools::chain;

use crate::{traits::IterValues, types::weapon_slot::WeaponSlot};

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
    derive_more::From,
)]
pub enum EquipmentSlot {
    Goggles,
    Helmet,
    Necklace,
    Trinket,
    Cloak,
    Belt,
    Ring1,
    Ring2,
    Gloves,
    Boots,
    Bracers,
    Armor,
    Weapon(WeaponSlot),
}

impl IterValues for EquipmentSlot {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            [
                Self::Goggles,
                Self::Helmet,
                Self::Necklace,
                Self::Trinket,
                Self::Cloak,
                Self::Belt,
                Self::Ring1,
                Self::Ring2,
                Self::Gloves,
                Self::Boots,
                Self::Bracers,
                Self::Armor,
            ],
            WeaponSlot::values().map(Self::Weapon)
        )
    }
}
