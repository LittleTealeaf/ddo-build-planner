//! Different item types
mod weapon;
mod shield;

pub use weapon::*;
pub use shield::*;

use enum_map::Enum;

/// The types that an item can be
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ItemType {
    /// Weapons
    Weapon(WeaponType),
    /// Shields
    Shield(ShieldType)
}
