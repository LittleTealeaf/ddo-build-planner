//! Different item types
mod shield;
mod weapon;

pub use shield::*;
pub use weapon::*;

use enum_map::Enum;

/// The types that an item can be
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ItemType {
    /// Weapons
    Weapon(WeaponType),
    /// Shields
    Shield(ShieldType),
}
