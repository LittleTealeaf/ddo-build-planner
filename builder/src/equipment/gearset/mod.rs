//! Describes a gearset

use std::collections::HashMap;

use super::item::Item;

/// Describes a specific gearset
#[derive(Debug, Clone)]
pub struct Gearset {
    items: HashMap<Gearset, Item>,
}
