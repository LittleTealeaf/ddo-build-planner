use crate::{
    attribute::{
        types::{WeaponHand, WeaponStat},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType},
};

/// Implements various trats for [`Attribute::BaseAttackBonus`]
pub struct _BaseAttackBonus;

impl GetBonuses for _BaseAttackBonus {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        Some(vec![Bonus::new(
            (WeaponHand::Both, WeaponStat::Attack).into(),
            BonusType::Stacking,
            value.into(),
            Attribute::BaseAttackBonus.into(),
            None,
        )])
    }
}
