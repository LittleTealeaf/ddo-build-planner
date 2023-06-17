//! Describes a single item

use serde::{Deserialize, Serialize};

use crate::bonus::{Bonus, BonusSource};

use super::ItemSlot;

pub mod types;

/// Describes a single item.
#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    slots: Vec<ItemSlot>,
}


impl Item {



    /// Returns a list of bonuses based on the slot it comes from
    pub fn get_bonuses(&self, slot: ItemSlot) -> Vec<Bonus> {
        let _source = BonusSource::from(slot);



        vec![]
    }
}
