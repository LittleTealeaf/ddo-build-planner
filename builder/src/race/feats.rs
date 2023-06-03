use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{
        types::{ArmorClass, Skill, WeaponHand, WeaponStat},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType},
    feat::Feat,
};

#[derive(PartialEq, Eq, Clone, Copy, Enum, Debug)]
pub enum RacialFeat {
    SmallSizeBonus,
}

impl Display for RacialFeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RacialFeat::SmallSizeBonus => write!(f, "Small Size Bonus"),
        }
    }
}

impl GetBonuses for RacialFeat {
    fn get_bonuses(&self, _: f32) -> Option<Vec<Bonus>> {
        match self {
            RacialFeat::SmallSizeBonus => Some(vec![
                Bonus::new(
                    (WeaponHand::Both, WeaponStat::Attack).into(),
                    BonusType::Size,
                    1f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::SmallSizeBonus)).into(),
                    None,
                ),
                Bonus::new(
                    ArmorClass::Bonus.into(),
                    BonusType::Size,
                    1f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::SmallSizeBonus)).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Hide.into(),
                    BonusType::Size,
                    4f32.into(),
                    Attribute::from(Feat::RacialFeat(RacialFeat::SmallSizeBonus)).into(),
                    None,
                ),
            ]),
        }
    }
}
