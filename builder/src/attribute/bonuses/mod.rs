//! Subsidary types used to specify [`Attribute`] values.
//!
//! [`Attribute`]: crate::attribute::Attribute
mod ability;
mod alignment;
mod armor_class;
mod damage_reduction;
mod energy_resistance;
mod immunity;
mod monster_type;
mod saving_throw;
mod sheltering;
mod skill;
mod spell_power;
mod spell_school;
mod weapon_attribute;
mod weapon_hand;
mod weapon_stat;

pub use ability::*;
pub use alignment::*;
pub use armor_class::*;
pub use damage_reduction::*;
pub use energy_resistance::*;
pub use immunity::*;
pub use monster_type::*;
pub use saving_throw::*;
pub use sheltering::*;
pub use skill::*;
pub use spell_power::*;
pub use spell_school::*;
pub use weapon_attribute::*;
pub use weapon_hand::*;
pub use weapon_stat::*;
