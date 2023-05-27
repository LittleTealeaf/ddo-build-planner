use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ArmorClass {
    Bonus,
    ArmorBonus,
    ShieldBonus,
    ArmorScalar,
    ShieldScalar,
    Scalar,
}

impl Display for ArmorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmorClass::Bonus => write!(f, "Armor Class"),
            ArmorClass::ArmorBonus => write!(f, "Armor AC"),
            ArmorClass::ShieldBonus => write!(f, "Shield AC"),
            ArmorClass::ArmorScalar => write!(f, "% Armor AC"),
            ArmorClass::ShieldScalar => write!(f, "% Shield AC"),
            ArmorClass::Scalar => write!(f, "% Armor Class"),
        }
    }
}

impl GetBonuses for ArmorClass {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            ArmorClass::ArmorScalar => Some(vec![Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::Stacking,
                (Attribute::from(ArmorClass::ArmorBonus), value).into(),
                Attribute::from(ArmorClass::ArmorScalar).into(),
                None,
            )]),
            ArmorClass::ShieldScalar => Some(vec![Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::Stacking,
                (Attribute::from(ArmorClass::ShieldBonus), value).into(),
                Attribute::from(ArmorClass::ShieldScalar).into(),
                None,
            )]),
            _ => None,
        }
    }
}

impl From<ArmorClass> for Attribute {
    fn from(value: ArmorClass) -> Self {
        Self::ArmorClass(value)
    }
}
