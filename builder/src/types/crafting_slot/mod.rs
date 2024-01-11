//! Describes the slot types for crafting on items

use serde::{Deserialize, Serialize};
use utils::public_modules;

public_modules!(augment);

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
pub enum CraftingSlot {
    Augment(AugmentSlot),
}
