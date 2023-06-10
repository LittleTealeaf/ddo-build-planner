//! Different item types
mod shield;
mod weapon;

pub use shield::*;
pub use weapon::*;


/// The types that an item can be
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ItemType {
    /// Weapons
    Weapon(WeaponType),
    /// Shields
    Shield(ShieldType),
}
