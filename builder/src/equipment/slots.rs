use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Describes the item slot that an item is put in
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Serialize, Deserialize, Debug)]
pub enum ItemSlot {
    /// Goggles slot
    Goggles,
    /// Helmet slot
    Helmet,
    /// Necklace slot
    Necklace,
    /// Trinket slot
    Trinket,
    /// Cloak slot
    Cloak,
    /// Belt slot
    Belt,
    /// Ring Ring slot
    Ring1,
    /// Gloves slot
    Gloves,
    /// Boots slot
    Boots,
    /// Left Ring slot
    Ring2,
    /// Bracers slot
    Bracers,
    /// Armor slot
    Armor,
    /// Main Hand
    MainHand,
    /// Off Hand
    OffHand,
}

impl Display for ItemSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemSlot::Goggles => write!(f, "Goggles"),
            ItemSlot::Helmet => write!(f, "Helmet"),
            ItemSlot::Necklace => write!(f, "Necklace"),
            ItemSlot::Trinket => write!(f, "Trinket"),
            ItemSlot::Cloak => write!(f, "Cloak"),
            ItemSlot::Belt => write!(f, "Belt"),
            ItemSlot::Ring1 => write!(f, "Right Ring"),
            ItemSlot::Gloves => write!(f, "Gloves"),
            ItemSlot::Boots => write!(f, "Boots"),
            ItemSlot::Ring2 => write!(f, "Left Ring"),
            ItemSlot::Bracers => write!(f, "Bracers"),
            ItemSlot::Armor => write!(f, "Armor"),
            ItemSlot::MainHand => write!(f, "MainHand"),
            ItemSlot::OffHand => write!(f, "OffHand"),
        }
    }
}
