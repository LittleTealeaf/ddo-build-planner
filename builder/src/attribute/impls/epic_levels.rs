use crate::{
    attribute::{types::SavingThrow, Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
};

/// Dummy struct for Epic Levels
pub struct _EpicLevel;

impl GetBonuses for _EpicLevel {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0f32).then(|| {
            vec![Bonus::new(
                SavingThrow::All.into(),
                BonusType::Stacking.into(),
                ((value + 1f32) / 2f32).floor().into(),
                Attribute::EpicLevel.into(),
                None,
            )]
        })
    }
}
