use serde::{Serialize, Deserialize};



/// Colored Augment Slot
#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
pub enum AugmentSlot {
    /// Colorless Augment Slot
    Colorless,
    /// Red Augment Slot
    Red,
    /// Blue Augment Slot
    Blue,
    /// Yellow Augment Slot
    Yellow,
    /// Purple Augment Slot
    Purple,
    /// Orange Augment Slot
    Orange,
    /// Green Augment Slot
    Green
}
