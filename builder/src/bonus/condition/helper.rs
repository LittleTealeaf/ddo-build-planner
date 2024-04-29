use utils::enums::StaticOptions;

use crate::types::{
    flag::{MainHandType, OffHandType},
    item_type::{ShieldType, WeaponType},
};

use super::{Condition, ConditionFold};

impl Condition {
    /// Asserts that the user is using a two-handed fighting weapon
    #[must_use]
    pub fn is_two_handed_fighting_stance() -> Self {
        WeaponType::TWO_HANDED_FIGHTING
            .map(|weapon| Self::has(MainHandType::Weapon(weapon)))
            .cond_any()
            .unwrap_or_default()
    }

    /// Asserts if there is a weapon in the main hand
    #[must_use]
    pub fn something_in_main_hand() -> Self {
        MainHandType::get_static()
            .map(Self::has)
            .cond_any()
            .unwrap_or_default()
    }

    /// Asserts if there is a weapon in the off hand
    #[must_use]
    pub fn something_in_off_hand() -> Self {
        OffHandType::get_static()
            .map(Self::has)
            .cond_any()
            .unwrap_or_default()
    }

    /// Asserts that the user is in a valid single weapon fighting stance
    #[must_use]
    pub fn is_single_weapon_fighting_stance() -> Self {
        WeaponType::SINGLE_HANDED_WEAPON
            .map(|weapon| Self::has(MainHandType::Weapon(weapon)))
            .cond_any()
            .unwrap_or_default()
            & [
                Self::has(OffHandType::Shield(ShieldType::Orb)),
                Self::has(OffHandType::RuneArm),
                !Self::something_in_off_hand(),
            ]
            .cond_any()
            .unwrap_or_default()
    }
}
