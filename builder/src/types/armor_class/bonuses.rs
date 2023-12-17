use crate::{
    attribute::{Attribute, DefaultBonuses},
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
    equipment::item::types::{ArmorType, ShieldType},
    types::{
        ability::Ability,
        flag::{Flag, OffHandType},
    },
};

use super::ArmorClass;

fn is_wearing_armor() -> Condition {
    Condition::has(Flag::ArmorType(ArmorType::Light).into())
        | Condition::has(Flag::ArmorType(ArmorType::Medium).into())
        | Condition::has(Flag::ArmorType(ArmorType::Heavy).into())
}

fn is_wielding_tower_shield() -> Condition {
    Condition::has(Attribute::from(Flag::OffHandType(OffHandType::Shield(
        ShieldType::TowerShield,
    ))))
}

impl DefaultBonuses for ArmorClass {
    type Iterator = Vec<Bonus>;

    fn get_default_bonuses() -> Self::Iterator {
        vec![
            // Dexterity Bonus to Armor Class
            Bonus::new(
                Attribute::ArmorClass(Self::Bonus),
                BonusType::AbilityModifier,
                Value::If {
                    condition: is_wearing_armor().into(),
                    if_true: Box::new(Value::If {
                        condition: is_wielding_tower_shield().into(),
                        if_true: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .min(Value::Attribute(Attribute::ArmorClass(Self::ArmorMaxDex)))
                            .min(Value::Attribute(Attribute::ArmorClass(Self::ShieldMaxDex)))
                            .into(),
                        if_false: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .min(Value::Attribute(Attribute::ArmorClass(Self::ArmorMaxDex)))
                            .into(),
                    }),
                    if_false: Box::new(Value::If {
                        condition: is_wielding_tower_shield().into(),
                        if_false: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .into(),
                        if_true: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .min(Value::Attribute(Attribute::ArmorClass(Self::ShieldMaxDex)))
                            .into(),
                    }),
                },
                BonusSource::Base,
                None,
            ),
            // Total Armor Class Bonus
            Bonus::new(
                Attribute::ArmorClass(Self::TotalArmorClass),
                BonusType::Standard,
                [
                    Value::Attribute(Attribute::ArmorClass(Self::Bonus)),
                    Value::Attribute(Attribute::ArmorClass(Self::NaturalArmor)),
                    Value::Attribute(Attribute::ArmorClass(Self::ShieldBonus))
                        * (Value::Value(1f32)
                            + Value::Attribute(Attribute::ArmorClass(Self::ShieldScalar))),
                    Value::Attribute(Attribute::ArmorClass(Self::ArmorBonus))
                        * (Value::Value(1f32)
                            + Value::Attribute(Attribute::ArmorClass(Self::ArmorScalar))),
                    Value::Value(10f32),
                ]
                .into_iter()
                .sum::<Value>()
                    * (Value::Value(1f32)
                        + Value::Attribute(Attribute::ArmorClass(Self::TotalScalar))),
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
