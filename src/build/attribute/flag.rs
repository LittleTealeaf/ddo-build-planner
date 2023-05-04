use crate::build::items::types::{WeaponType, ArmorType};

use super::{ability::Ability, saving_throw::SavingThrow};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flag {
    WeaponProficiency(WeaponType),
    ArmorProficiency(ArmorType),
    AbilityForAttack(Ability),
    AbilityForDamage(Ability),
    AbilityForSavingThrow(Ability, SavingThrow),
}
