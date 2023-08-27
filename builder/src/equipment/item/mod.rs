//! Describes items

pub mod stats;
pub mod types;
pub mod damage_dice;
pub mod bind_status;

use serde::{Deserialize, Serialize};

use self::bind_status::ItemBindStatus;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    name: String,
    bind_status: ItemBindStatus,
}
