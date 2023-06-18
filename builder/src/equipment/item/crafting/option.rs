use serde::{Deserialize, Serialize};

use crate::bonus::TemplateBonus;

use super::ItemCrafting;

/// Describes a crafting option for the user
#[derive(Deserialize, Serialize, Clone)]
pub struct CraftingOption {
    name: String,
    bonuses: Vec<TemplateBonus>,
    sub_crafting: Vec<ItemCrafting>
}
