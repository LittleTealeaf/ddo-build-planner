use serde::{Deserialize, Serialize};

use crate::bonus::BonusTemplate;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchantment {
    name: String,
    description: String,
    bonuses: Vec<BonusTemplate>,
}
