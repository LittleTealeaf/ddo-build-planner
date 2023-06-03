use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
};

/// Represents different attributes that relate to Armor Class
#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum)]
pub enum ArmorClass {
    /// Flat bonuses to armor class
    Bonus,
    /// Bonuses to armor class from armor
    ArmorBonus,
    /// Bonuses to armor class from shields
    ShieldBonus,
    /// Scaling for [`ArmorClass::ArmorBonus`]
    ArmorScalar,
    /// Scaling for [`ArmorClass::ShieldBonus`]
    ShieldScalar,
    /// Scaling for [`ArmorClass::Bonus`]
    Scalar,
    /// Natural Armor
    NaturalArmor,
    /// Calcualted Dex Bonus used in calculations.
    ///
    /// If you are trying to add max dex bonuses, use [`ArmorClass::MaxDexBonus`]. This attribute is derived from that and [`ArmorClass::TowerShieldMaxDexBonus`].
    CalculatedMaxDexBonus,
    /// Max Dex Bonus
    MaxDexBonus,
    /// Max Dex Bonus for Tower Shields
    TowerShieldMaxDexBonus,
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
            ArmorClass::NaturalArmor => write!(f, "Natural Armor"),
            ArmorClass::MaxDexBonus => write!(f, "Max Dexterity Bonus"),
            ArmorClass::TowerShieldMaxDexBonus => write!(f, "Max Dexterity Bonus (Tower Shields)"),
            ArmorClass::CalculatedMaxDexBonus => write!(f, "Calculated Max Dex Bonus")
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
            ArmorClass::NaturalArmor => Some(vec![Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::Stacking,
                value.into(),
                Attribute::from(ArmorClass::NaturalArmor).into(),
                None,
            )]),
            ArmorClass::MaxDexBonus => Some(vec![
                Bonus::new(
                    ArmorClass::CalculatedMaxDexBonus.into(),
                    BonusType::Stacking,
                    value.into(),
                    Attribute::from(ArmorClass::MaxDexBonus).into(),
                    None // TODO: check if not weielding a tower shield and TowerShieldMaxDexBonus is less than the value
                )
            ]),
            _ => None,
        }
    }
}

impl From<ArmorClass> for Attribute {
    fn from(value: ArmorClass) -> Self {
        Self::ArmorClass(value)
    }
}
