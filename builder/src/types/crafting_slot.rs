//! Describes the slot types for crafting on items

use serde::{Deserialize, Serialize};
use utils::public_modules;

public_modules!(augment, dino_bone);

/// Describes the slot available for the item
#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
pub enum CraftingSlot {
    /// Augment Slots
    Augment(AugmentSlot),
    /// Dinosaur Bone Crafting Slots
    DinoBone(DinoBoneSlot),
}
