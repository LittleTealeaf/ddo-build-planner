//! Describes items

use rust_decimal::Decimal;

use crate::types::gear_slot::GearSlot;

pub mod template;

/// Describes an item
#[derive(Clone, Debug)]
pub struct Item {
    name: String,
    description: String,
    minimum_level: Decimal,
    slots: Vec<GearSlot>,
    prevent_slots: Vec<GearSlot>,
}
