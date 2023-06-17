use serde::{Deserialize, Serialize};

use crate::bonus::TemplateBonus;

/// Describes a crafting option for the user
#[derive(Deserialize, Serialize, Clone)]
pub struct CraftingOption {
    name: String,
    bonuses: Vec<TemplateBonus>
}
