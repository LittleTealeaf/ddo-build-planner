//! Different item types
mod weapon;
mod shield;

pub use weapon::*;
pub use shield::*;

use enum_map::Enum;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ItemType {
    Weapon(WeaponType),
    Shield(ShieldType)
}
