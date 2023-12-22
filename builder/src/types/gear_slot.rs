//! Describes a specific gear slot that an item goes into

use serde::{Deserialize, Serialize};

/// Describes the slot that an item can go into
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd, Serialize, Deserialize)]
pub enum GearSlot {
    /// Helmet Slot
    Helmet,
    /// Goggles Slot
    Eyes,
    /// Necklace Slot
    Neck,
    /// Trinket Slot
    Trinket,
    /// Right Ring Slot
    Ring1,
    /// Left Ring Slot
    Ring2,
    /// Belt Slot
    Belt,
    /// Cloak Slot
    Cloak,
    /// Armor Slot
    Armor,
    /// Goggles Slot
    Goggles,
    /// Boots Slot
    Boots,
    /// Gloves Slot
    Gloves,
    /// Quiver Slot
    Quiver,
    /// Bracers Slot
    Bracers,
    /// Main Hand Slot
    MainHand,
    /// Off Hand Slot
    OffHand
}
