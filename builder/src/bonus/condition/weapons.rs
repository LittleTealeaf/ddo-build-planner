use crate::types::{flag::MainHandType, item_type::WeaponType};

use super::{Condition, ConditionFold};

impl Condition {
    /// Asserts that the user is using a two-handed fighting weapon
    #[must_use]
    pub fn is_two_handed_fighting() -> Self {
        [
            WeaponType::GreatAxe,
            WeaponType::GreatClub,
            WeaponType::Quarterstaff,
            WeaponType::Falchion,
        ]
        .map(|weapon| Self::has(MainHandType::Weapon(weapon)))
        .cond_any()
        .unwrap_or_default()
    }
}
