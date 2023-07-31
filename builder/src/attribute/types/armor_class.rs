use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{
        flags::{Flag, OffHandType},
        types::Ability,
        Attribute, DefaultBonuses,
    },
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
    equipment::item::types::{ArmorType, ShieldType},
};

/// Represents different attributes that relate to Armor Class
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArmorClass {
    /// Bonuses to armor class from armor
    ArmorBonus,
    /// Bonuses to armor class from shields
    ShieldBonus,
    /// Scaling for [`ArmorClass::ArmorBonus`]
    ArmorScalar,
    /// Scaling for [`ArmorClass::ShieldBonus`]
    ShieldScalar,
    /// Natural Armor
    NaturalArmor,
    /// Max Dex Bonus for Armor
    ArmorMaxDexBonus,
    /// Max Dex Bonus for Tower Shield
    ShieldMaxDexBonus,
    /// Calculated Max Dex Bonus
    ///
    /// DO NOT MANUALLY ADD BONUSES TO THIS ATTRIBUTE.
    CalculatedMaxDexBonus,
    /// Flat bonuses to armor class
    Bonus,
    /// Scaling for [`ArmorClass::Bonus`]
    Scalar,
}

impl Display for ArmorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bonus => write!(f, "Armor Class"),
            Self::ArmorBonus => write!(f, "Armor AC"),
            Self::ShieldBonus => write!(f, "Shield AC"),
            Self::ArmorScalar => write!(f, "% Armor AC"),
            Self::ShieldScalar => write!(f, "% Shield AC"),
            Self::Scalar => write!(f, "% Armor Class"),
            Self::NaturalArmor => write!(f, "Natural Armor"),
            Self::CalculatedMaxDexBonus => write!(f, "Calculated Max Dex Bonus"),
            Self::ArmorMaxDexBonus => write!(f, "Armor Max Dex Bonus"),
            Self::ShieldMaxDexBonus => write!(f, "Tower Shield Max Dex Bonus"),
        }
    }
}

fn is_wearing_armor() -> Condition {
    Condition::Any(vec![
        Condition::has(Flag::ArmorType(ArmorType::Light).into()),
        Condition::has(Flag::ArmorType(ArmorType::Medium).into()),
        Condition::has(Flag::ArmorType(ArmorType::Heavy).into()),
    ])
}

fn is_wielding_tower_shield() -> Condition {
    Condition::has(Flag::OffHandType(OffHandType::Shield(ShieldType::TowerShield)).into())
}

impl DefaultBonuses for ArmorClass {
    type Iterator = [Bonus; 4];

    fn get_default_bonuses() -> Self::Iterator {
        [
            // Armor class bonus scaled
            Bonus::new(
                Self::Bonus.into(),
                BonusType::Stacking,
                Value::Sum(vec![
                    Value::Product(vec![
                        Attribute::from(Self::ArmorBonus).into(),
                        Attribute::from(Self::ArmorScalar).into(),
                    ]),
                    Value::Product(vec![
                        Attribute::from(Self::ShieldBonus).into(),
                        Attribute::from(Self::ShieldScalar).into(),
                    ]),
                    Attribute::from(Self::NaturalArmor).into(),
                ]),
                BonusSource::Base,
                None,
            ),
            // Armor class bonus scaled from shield
            // Max Dex Bonus from armor
            Bonus::new(
                Attribute::ArmorClass(Self::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(Self::ArmorMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    is_wearing_armor(),
                    Condition::NotAll(vec![
                        is_wielding_tower_shield(),
                        Condition::GreaterThan(
                            Attribute::from(Self::ArmorMaxDexBonus).into(),
                            Attribute::from(Self::ShieldMaxDexBonus).into(),
                        ),
                    ]),
                ])),
            ),
            // Max dex bonus from shield
            Bonus::new(
                Attribute::ArmorClass(Self::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(Self::ShieldMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    is_wielding_tower_shield(),
                    Condition::NotAll(vec![
                        is_wearing_armor(),
                        Condition::GreaterThan(
                            Attribute::from(Self::ShieldMaxDexBonus).into(),
                            Attribute::from(Self::ArmorMaxDexBonus).into(),
                        ),
                    ]),
                ])),
            ),
            Bonus::new(
                Self::Bonus.into(),
                BonusType::AbilityModifier,
                Value::If(
                    Condition::has(Attribute::ArmorClass(Self::CalculatedMaxDexBonus)).into(),
                    Value::Min(vec![
                        Attribute::AbilityModifier(Ability::Dexterity).into(),
                        Attribute::ArmorClass(Self::CalculatedMaxDexBonus).into(),
                    ])
                    .into(),
                    Value::from(Attribute::AbilityModifier(Ability::Dexterity)).into(),
                ),
                BonusSource::Base,
                None,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(ArmorClass);
}
