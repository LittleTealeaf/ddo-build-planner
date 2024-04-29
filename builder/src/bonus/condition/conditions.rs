use crate::types::{flag::MainHandType, item_type::WeaponType};

use super::{Condition, ConditionFold};

impl Condition {
    /// Asserts that the stance is a two handed fighting stance
    #[must_use]
    pub fn stance_two_handed_fighting() -> Self {
        WeaponType::TWO_HANDED_FIGHTING
            .map(|weapon| Self::has(MainHandType::Weapon(weapon)))
            .cond_any()
            .unwrap_or(Self::FALSE)
    }
}
