use serde::{Serialize, Deserialize};



#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
pub enum AugmentSlot {
    Colorless,
    Red,
    Blue,
    Yellow,
    Purple,
    Orange,
    Green
}
