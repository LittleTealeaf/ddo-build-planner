//! Describes a single item

use serde::{Deserialize, Serialize};

use crate::bonus::Bonus;

use super::ItemSlot;

pub mod types;

/// Describes a single item.
#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    slots: Vec<ItemSlot>,
}


impl Item {



    pub fn get_bonuses(&self, slot: ItemSlot) -> Vec<Bonus> {


        vec![]
    }
}
