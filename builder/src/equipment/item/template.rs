//! Describes a template stored within the data crate, used to instantiate a new item instance

use serde::{Deserialize, Serialize};

use crate::types::{crafting_slot::CraftingSlot, gear_slot::GearSlot};

/// A template for creating [`Item`] instances. This is stored in the data crate to include in the
/// bianry
///
/// [`Item`]: super::Item
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ItemTemplate {
    name: String,
    description: String,
    minimum_level: i32,
    slots: Vec<GearSlot>,
    prevent_slots: Vec<GearSlot>,
    crafting_slots: Vec<CraftingSlot>,
}

impl ItemTemplate {
    /// Returns a reference to the name of this [`ItemTemplate`].
    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Returns a reference to the description of this [`ItemTemplate`].
    #[must_use]
    pub const fn description(&self) -> &String {
        &self.description
    }

    /// Returns the get minimum level of this [`ItemTemplate`].
    #[must_use]
    pub const fn minimum_level(&self) -> i32 {
        self.minimum_level
    }

    /// Returns a reference to the get slots of this [`ItemTemplate`].
    #[must_use]
    pub const fn slots(&self) -> &Vec<GearSlot> {
        &self.slots
    }

    /// Returns a reference to the get prevent slots of this [`ItemTemplate`].
    #[must_use]
    pub const fn prevent_slots(&self) -> &Vec<GearSlot> {
        &self.prevent_slots
    }

    /// Returns a reference to the crafting slots of this [`ItemTemplate`].
    #[must_use]
    pub const fn crafting_slots(&self) -> &Vec<CraftingSlot> {
        &self.crafting_slots
    }

    /// Returns a mutable reference to the mut slots of this [`ItemTemplate`].
    pub fn slots_mut(&mut self) -> &mut Vec<GearSlot> {
        &mut self.slots
    }

    /// Returns a mutable reference to the mut prevent slots of this [`ItemTemplate`].
    pub fn prevent_slots_mut(&mut self) -> &mut Vec<GearSlot> {
        &mut self.prevent_slots
    }

    /// Returns a mutable reference to the crafting slots of this [`ItemTemplate`]
    pub fn crafting_slots_mut(&mut self) -> &mut Vec<CraftingSlot> {
        &mut self.crafting_slots
    }

    /// Sets the name of this [`ItemTemplate`].
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the description of this [`ItemTemplate`].
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Sets the minimum level of this [`ItemTemplate`].
    pub fn set_minimum_level(&mut self, minimum_level: i32) {
        self.minimum_level = minimum_level;
    }

    /// Sets the slots of this [`ItemTemplate`].
    pub fn set_slots(&mut self, slots: Vec<GearSlot>) {
        self.slots = slots;
    }

    /// Sets the prevent slots of this [`ItemTemplate`].
    pub fn set_prevent_slots(&mut self, prevent_slots: Vec<GearSlot>) {
        self.prevent_slots = prevent_slots;
    }

    /// Sets the crafting slots of this [`ItemTemplate`].
    pub fn set_crafting_slots(&mut self, crafting_slots: Vec<CraftingSlot>) {
        self.crafting_slots = crafting_slots;
    }
}
