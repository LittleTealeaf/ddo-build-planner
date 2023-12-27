//! Application Starting Point
use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition, Value},
    breakdowns::Breakdowns,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        flag::{Flag, OffHandType},
        item::{ArmorType, ShieldType},
        player_class::PlayerClass,
        race::Race,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

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

fn main() {
    let mut breakdowns = Breakdowns::new();

    println!("Adding Bonuses");

    println!(
        "{}",
        ron::to_string(
            &([
                Value::Attribute(Attribute::ArmorClass(ArmorClass::Bonus)),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::NaturalArmor)),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::ShieldBonus))
                    * (Value::Value(1f32)
                        + Value::Attribute(Attribute::ArmorClass(ArmorClass::ShieldScalar))),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::ArmorBonus))
                    * (Value::Value(1f32)
                        + Value::Attribute(Attribute::ArmorClass(ArmorClass::ArmorScalar))),
                Value::Value(10f32),
                Value::If {
                    condition: is_wearing_armor().into(),
                    if_true: Box::new(Value::If {
                        condition: is_wielding_tower_shield().into(),
                        if_true: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .min(Value::Attribute(Attribute::ArmorClass(
                                ArmorClass::ArmorMaxDex
                            )))
                            .min(Value::Attribute(Attribute::ArmorClass(
                                ArmorClass::ShieldMaxDex
                            )))
                            .into(),
                        if_false: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .min(Value::Attribute(Attribute::ArmorClass(
                                ArmorClass::ArmorMaxDex
                            )))
                            .into(),
                    }),
                    if_false: Box::new(Value::If {
                        condition: is_wielding_tower_shield().into(),
                        if_false: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .into(),
                        if_true: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                            .min(Value::Attribute(Attribute::ArmorClass(
                                ArmorClass::ShieldMaxDex
                            )))
                            .into(),
                    }),
                }
            ]
            .into_iter()
            .sum::<Value>()
                * (Value::Value(1f32)
                    + Value::Attribute(Attribute::ArmorClass(ArmorClass::TotalScalar))))
        )
        .unwrap()
    );

    breakdowns.insert_bonuses([Bonus::new(
        Ability::All.into(),
        BonusType::Stacking,
        10f32.into(),
        BonusSource::Custom(10),
        None,
    )]);

    breakdowns.insert_bonuses([
        Bonus::new(
            PlayerClass::FavoredSoul.into(),
            BonusType::Stacking,
            10f32.into(),
            0.into(),
            None,
        ),
        // Bonus::flag(OffHandType::from(ShieldType::TowerShield).into(), 0.into()),
        Bonus::flag(Race::Gnome.into(), 0.into()),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ShieldMaxDex),
            BonusType::Stacking,
            5f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ArmorMaxDex),
            BonusType::Stacking,
            10f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::All.into(),
            BonusType::Stacking,
            8f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Dexterity.into(),
            BonusType::Stacking,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Intelligence.into(),
            BonusType::Stacking,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Enhancement,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Insightful,
            10f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            (WeaponHand::Main, WeaponStat::Attack).into(),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(Ability::Intelligence).into(),
            2.into(),
            None,
        ),
    ]);

    for (attr, val) in breakdowns.iter_attributes() {
        println!("{attr}: {val}");
    }
}
