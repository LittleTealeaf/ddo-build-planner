use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{
        flags::{Flag, OffHandType},
        types::Ability,
        Attribute, DefaultBonuses, GetBonuses,
    },
    bonus::{Bonus, BonusSource, BonusType, Condition},
    item::types::ShieldType,
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
    /// Calculated Max Dex Bonus
    ///
    /// DO NOT MANUALLY ADD BONUSES TO THIS ATTRIBUTE.
    CalculatedMaxDexBonus,
    /// Max Dex Bonus for Armor
    ArmorMaxDexBonus,
    /// Max Dex Bonus for Tower Shield
    ShieldMaxDexBonus,
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
            _ => None,
        }
    }
}

// TODO: Impelemnt
const IS_WEARING_ARMOR: Condition = Condition::Any(vec![]);

const IS_WIELDING_SHIELD: Condition = Condition::Has(Attribute::Flag(Flag::OffHandType(
    OffHandType::Shield(ShieldType::TowerShield),
)));

impl DefaultBonuses for ArmorClass {
    fn get_default_bonuses() -> Vec<Bonus> {
        vec![
            Bonus::new(
                Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(ArmorClass::ArmorMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    IS_WEARING_ARMOR,
                    Condition::NotAll(vec![
                        IS_WIELDING_SHIELD,
                        Condition::GreaterThan(
                            ArmorClass::ArmorMaxDexBonus.into(),
                            ArmorClass::ShieldMaxDexBonus.into(),
                        ),
                    ]),
                ])),
            ),
            Bonus::new(
                Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(ArmorClass::ShieldMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    IS_WIELDING_SHIELD,
                    Condition::NotAll(vec![
                        IS_WEARING_ARMOR,
                        Condition::GreaterThan(
                            ArmorClass::ShieldMaxDexBonus.into(),
                            ArmorClass::ArmorMaxDexBonus.into(),
                        ),
                    ]),
                ])),
            ),
            // If max dex bonus is higher than value
            Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::AbilityModifier,
                Attribute::Ability(Ability::Dexterity).into(),
                BonusSource::Base,
                Some(Condition::Any(vec![
                    Condition::NotHave(ArmorClass::CalculatedMaxDexBonus.into()),
                    Condition::LessThan(
                        Attribute::Ability(Ability::Dexterity),
                        Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                    ),
                    Condition::EqualTo(
                        Attribute::Ability(Ability::Dexterity),
                        Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                    ),
                ])),
            ),
            // If max dex bonus is lower than value
            Bonus::new(
                ArmorClass::Bonus.into(),
                BonusType::AbilityModifier,
                Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus).into(),
                BonusSource::Base,
                Some(Condition::All(vec![
                    Condition::Has(ArmorClass::CalculatedMaxDexBonus.into()),
                    Condition::GreaterThan(
                        Attribute::Ability(Ability::Dexterity),
                        Attribute::ArmorClass(ArmorClass::CalculatedMaxDexBonus),
                    ),
                ])),
            ),
        ]
    }
}

impl From<ArmorClass> for Attribute {
    fn from(value: ArmorClass) -> Self {
        Self::ArmorClass(value)
    }
}
