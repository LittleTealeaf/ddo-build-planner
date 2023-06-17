//! Provides the user with the ability to choose certain crafting features on an item.

use serde::{Serialize, Deserialize};

mod option;

pub use option::*;


/// Indicates the crafting options able to be crafted on an item.
#[derive(Serialize, Deserialize)]
pub enum ItemCrafting {
    /// Customized Item Crafting
    Custom()
}
