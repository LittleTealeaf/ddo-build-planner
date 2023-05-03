use crate::build::items::types::{WeaponType, ArmorType};

use super::ability::Ability;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    WeaponProficiency(WeaponType),
    ArmorProficiency(ArmorType),
    AbilityForAttack(Ability),
    AbilityForDamage(Ability),
}
