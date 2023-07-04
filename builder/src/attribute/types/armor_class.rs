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
            ArmorClass::Bonus => write!(f, "Armor Class"),
            ArmorClass::ArmorBonus => write!(f, "Armor AC"),
            ArmorClass::ShieldBonus => write!(f, "Shield AC"),
            ArmorClass::ArmorScalar => write!(f, "% Armor AC"),
            ArmorClass::ShieldScalar => write!(f, "% Shield AC"),
            ArmorClass::Scalar => write!(f, "% Armor Class"),
            ArmorClass::NaturalArmor => write!(f, "Natural Armor"),
            ArmorClass::CalculatedMaxDexBonus => write!(f, "Calculated Max Dex Bonus"),
            ArmorClass::ArmorMaxDexBonus => write!(f, "Armor Max Dex Bonus"),
            ArmorClass::ShieldMaxDexBonus => write!(f, "Tower Shield Max Dex Bonus"),
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
    fn get_default_bonuses() -> Vec<Bonus> {
        vec![
            // Armor class bonus scaled
            Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::Stacking,
                Value::Sum(vec![
                    Value::Product(vec![
                        Attribute::from(ArmorClass::ArmorBonus).into(),
                        Attribute::from(ArmorClass::ArmorScalar).into(),
                    ]),
                    Value::Product(vec![
                        Attribute::from(ArmorClass::ShieldBonus).into(),
                        Attribute::from(ArmorClass::ShieldScalar).into(),
                    ]),
                    Attribute::from(ArmorClass::NaturalArmor).into(),
                ]),
                BonusSource::Base,
                None,
            ),
            // Armor class bonus scaled from shield
            // Max Dex Bonus from armor
            Bonus::new(
                Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(ArmorClass::ArmorMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    is_wearing_armor(),
                    Condition::NotAll(vec![
                        is_wielding_tower_shield(),
                        Condition::GreaterThan(
                            Attribute::from(ArmorClass::ArmorMaxDexBonus).into(),
                            Attribute::from(ArmorClass::ShieldMaxDexBonus).into(),
                        ),
                    ]),
                ])),
            ),
            // Max dex bonus from shield
            Bonus::new(
                Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(ArmorClass::ShieldMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    is_wielding_tower_shield(),
                    Condition::NotAll(vec![
                        is_wearing_armor(),
                        Condition::GreaterThan(
                            Attribute::from(ArmorClass::ShieldMaxDexBonus).into(),
                            Attribute::from(ArmorClass::ArmorMaxDexBonus).into(),
                        ),
                    ]),
                ])),
            ),
            // If there is a max dex bonus
            Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::AbilityModifier,
                Value::Min(vec![
                    Attribute::AbilityModifier(Ability::Dexterity).into(),
                    Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus).into(),
                ]),
                BonusSource::Base,
                Some(Condition::has(Attribute::ArmorClass(
                    ArmorClass::CalculatedMaxDexBonus,
                ))),
            ),
            // If there is not a max dex bonus
            Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Dexterity).into(),
                BonusSource::Base,
                Some(Condition::not_have(
                    ArmorClass::CalculatedMaxDexBonus.into(),
                )),
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
