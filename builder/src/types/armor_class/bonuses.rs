use crate::{
    attribute::{
        flags::{Flag, OffHandType},
        Attribute, DefaultBonuses,
    },
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
    equipment::item::types::{ArmorType, ShieldType},
    types::Ability,
};

use super::ArmorClass;

fn is_wearing_armor() -> Condition {
    Condition::has(Flag::ArmorType(ArmorType::Light).into())
        | Condition::has(Flag::ArmorType(ArmorType::Medium).into())
        | Condition::has(Flag::ArmorType(ArmorType::Heavy).into())
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
                (Value::from(Attribute::from(Self::ArmorBonus))
                    * Value::from(Attribute::from(Self::ArmorScalar)))
                    + (Value::from(Attribute::from(Self::ShieldBonus))
                        * Value::from(Attribute::from(Self::ShieldScalar)))
                    + Attribute::from(Self::NaturalArmor).into(),
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
                Some(
                    is_wearing_armor()
                        & (is_wielding_tower_shield()
                            ^ Condition::GreaterThan(
                                Attribute::from(Self::ArmorMaxDexBonus).into(),
                                Attribute::from(Self::ShieldMaxDexBonus).into(),
                            )),
                ),
            ),
            // Max dex bonus from shield
            Bonus::new(
                Attribute::ArmorClass(Self::CalculatedMaxDexBonus),
                BonusType::Stacking,
                Attribute::ArmorClass(Self::ShieldMaxDexBonus).into(),
                BonusSource::Base,
                Some(
                    is_wielding_tower_shield()
                        & (is_wearing_armor()
                            ^ Condition::GreaterThan(
                                Attribute::from(Self::ShieldMaxDexBonus).into(),
                                Attribute::from(Self::ArmorMaxDexBonus).into(),
                            )),
                ),
            ),
            Bonus::new(
                Self::Bonus.into(),
                BonusType::AbilityModifier,
                Value::If {
                    condition: Condition::has(Attribute::ArmorClass(Self::CalculatedMaxDexBonus))
                        .into(),
                    if_true: Value::from(Attribute::AbilityModifier(Ability::Dexterity))
                        .min(Value::from(Attribute::ArmorClass(
                            Self::CalculatedMaxDexBonus,
                        )))
                        .into(),
                    if_false: Value::from(Attribute::AbilityModifier(Ability::Dexterity)).into(),
                },
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