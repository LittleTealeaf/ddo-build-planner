//! Describes a single item

use serde::{Deserialize, Serialize};

use crate::bonus::Bonus;

use super::ItemSlot;

mod material;
pub mod types;
pub mod crafting;

pub use material::*;

/// Describes a single item.
#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    minimum_level: u8,
    bonses: Vec<Bonus>,
    slots: Vec<ItemSlot>,
    anti_slots: Vec<ItemSlot>,
    material: ItemMaterial,
}
