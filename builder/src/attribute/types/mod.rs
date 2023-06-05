//! Subsidary types used to specify [`Attribute`] values.
//!
//! [`Attribute`]: crate::attribute::Attribute
mod ability;
mod armor_class;
mod damage_reduction;
mod saving_throw;
mod sheltering;
mod skill;
mod spell_power;
mod spell_school;
mod weapon_hand;
mod weapon_attribute;
mod weapon_stat;
mod immunity;
mod monster_type;
mod energy_resistance;
mod alignment;

pub use ability::*;
pub use armor_class::*;
pub use damage_reduction::*;
pub use saving_throw::*;
pub use sheltering::*;
pub use skill::*;
pub use spell_power::*;
pub use spell_school::*;
pub use weapon_hand::*;
pub use weapon_attribute::*;
pub use weapon_stat::*;
pub use immunity::*;
pub use monster_type::*;
pub use energy_resistance::*;
pub use alignment::*;