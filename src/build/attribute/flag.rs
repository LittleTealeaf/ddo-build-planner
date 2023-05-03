use crate::build::items::types::{WeaponType, ArmorType};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    WeaponProficiency(WeaponType),
    ArmorProficiency(ArmorType),
}
