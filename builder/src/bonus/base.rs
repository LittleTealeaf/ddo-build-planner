use itertools::chain;

use crate::{
    attribute::Attribute,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        damage_type::DamageType,
        flag::{Flag, OffHandType},
        health::Health,
        item::{ArmorType, ShieldType},
        player_class::PlayerClass,
        saving_throw::SavingThrow,
        skill::Skill,
        spell_points::SpellPoints,
        spell_power::SpellPower,
    },
};

use super::{Bonus, BonusSource, BonusType, Condition, Value};

/// Returns all base bonuses that are to be included by default.
pub fn get_base_bonuses() -> impl Iterator<Item = Bonus> {
    chain!(
        ability_bonuses(),
        saving_throw(),
        spell_power_skills(),
        spell_power_universal(),
        skill(),
        health(),
        spell_points(),
    )
}

fn ability_bonuses() -> impl IntoIterator<Item = Bonus> {
    Ability::ABILITIES.into_iter().flat_map(|ability| {
        [
            Bonus::new(
                Attribute::Ability(ability),
                BonusType::Stacking,
                8f32.into(),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                Attribute::AbilityModifier(ability),
                BonusType::Stacking,
                ((Value::Attribute(Attribute::Ability(ability)) - 10f32.into()) / 2f32.into())
                    .floor(),
                BonusSource::Base,
                None,
            ),
        ]
    })
}

fn saving_throw() -> impl IntoIterator<Item = Bonus> {
    [
        (Ability::Dexterity, SavingThrow::Reflex),
        (Ability::Constitution, SavingThrow::Fortitude),
        (Ability::Wisdom, SavingThrow::Will),
    ]
    .into_iter()
    .map(|(ability, saving_throw)| {
        Bonus::new(
            saving_throw.into(),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability).into(),
            BonusSource::Base,
            None,
        )
    })
}

fn spell_power_skills() -> impl IntoIterator<Item = Bonus> {
    [
        (Skill::Heal, DamageType::Positive),
        (Skill::Heal, DamageType::Negative),
        (Skill::Perform, DamageType::Sonic),
        (Skill::Spellcraft, DamageType::Acid),
        (Skill::Spellcraft, DamageType::Cold),
        (Skill::Spellcraft, DamageType::Electric),
        (Skill::Spellcraft, DamageType::Fire),
        (Skill::Spellcraft, DamageType::Force),
        (Skill::Spellcraft, DamageType::Light),
        (Skill::Spellcraft, DamageType::Poison),
    ]
    .into_iter()
    .map(|(skill, damage_type)| {
        Bonus::new(
            Attribute::SpellPower(SpellPower::Damage(damage_type)),
            BonusType::Stacking,
            Attribute::Skill(skill).into(),
            BonusSource::Base,
            None,
        )
    })
}

fn skill() -> impl IntoIterator<Item = Bonus> {
    [
        (Ability::Dexterity, Skill::Balance),
        (Ability::Charisma, Skill::Bluff),
        (Ability::Constitution, Skill::Concentration),
        (Ability::Charisma, Skill::Diplomacy),
        (Ability::Intelligence, Skill::DisableDevice),
        (Ability::Charisma, Skill::Haggle),
        (Ability::Wisdom, Skill::Heal),
        (Ability::Dexterity, Skill::Hide),
        (Ability::Charisma, Skill::Intimidate),
        (Ability::Strength, Skill::Jump),
        (Ability::Wisdom, Skill::Listen),
        (Ability::Dexterity, Skill::MoveSilently),
        (Ability::Dexterity, Skill::OpenLock),
        (Ability::Charisma, Skill::Perform),
        (Ability::Intelligence, Skill::Repair),
        (Ability::Intelligence, Skill::Search),
        (Ability::Intelligence, Skill::Spellcraft),
        (Ability::Wisdom, Skill::Spot),
        (Ability::Strength, Skill::Swim),
        (Ability::Dexterity, Skill::Tumble),
        (Ability::Charisma, Skill::UseMagicalDevice),
    ]
    .into_iter()
    .map(|(ability, skill)| {
        Bonus::new(
            Attribute::Skill(skill),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability).into(),
            BonusSource::Base,
            None,
        )
    })
}

fn armor_class() -> impl IntoIterator<Item = Bonus> {
    let is_wearing_armor = Condition::has(Attribute::from(Flag::ArmorType(ArmorType::Light)))
        | Condition::has(Flag::ArmorType(ArmorType::Medium).into())
        | Condition::has(Flag::ArmorType(ArmorType::Heavy).into());

    let is_wielding_tower_shield = Condition::has(Attribute::from(Flag::OffHandType(
        OffHandType::Shield(ShieldType::TowerShield),
    )));

    [
        // Dexterity Bonus to Armor Class
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::Bonus),
            BonusType::AbilityModifier,
            Value::If {
                condition: is_wearing_armor.into(),
                if_true: Box::new(Value::If {
                    condition: is_wielding_tower_shield.clone().into(),
                    if_true: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                        .min(Value::Attribute(Attribute::ArmorClass(
                            ArmorClass::ArmorMaxDex,
                        )))
                        .min(Value::Attribute(Attribute::ArmorClass(
                            ArmorClass::ShieldMaxDex,
                        )))
                        .into(),
                    if_false: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                        .min(Value::Attribute(Attribute::ArmorClass(
                            ArmorClass::ArmorMaxDex,
                        )))
                        .into(),
                }),
                if_false: Box::new(Value::If {
                    condition: is_wielding_tower_shield.into(),
                    if_false: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                        .into(),
                    if_true: Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                        .min(Value::Attribute(Attribute::ArmorClass(
                            ArmorClass::ShieldMaxDex,
                        )))
                        .into(),
                }),
            },
            BonusSource::Base,
            None,
        ),
        // Total Armor Class Bonus
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::TotalArmorClass),
            BonusType::Standard,
            [
                Value::Attribute(Attribute::ArmorClass(ArmorClass::Bonus)),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::NaturalArmor)),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::ShieldBonus))
                    * (Value::Value(1f32)
                        + Value::Attribute(Attribute::ArmorClass(ArmorClass::ShieldScalar))),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::ArmorBonus))
                    * (Value::Value(1f32)
                        + Value::Attribute(Attribute::ArmorClass(ArmorClass::ArmorScalar))),
                Value::Value(10f32),
            ]
            .into_iter()
            .sum::<Value>()
                * (Value::Value(1f32)
                    + Value::Attribute(Attribute::ArmorClass(ArmorClass::TotalScalar))),
            BonusSource::Base,
            None,
        ),
    ]
}

fn health() -> impl IntoIterator<Item = Bonus> {
    [
        Bonus::new(
            Attribute::Health(Health::Bonus),
            BonusType::Stacking,
            Value::from(Attribute::Health(Health::Base))
                * (Value::from(Attribute::Health(Health::BaseModifier)) + Value::from(1f32)),
            BonusSource::Base,
            None,
        ),
        Bonus::new(
            Attribute::Health(Health::Total),
            BonusType::Stacking,
            Value::from(Attribute::Health(Health::Bonus))
                * (Value::from(Attribute::Health(Health::Modifier)) + Value::from(1f32)),
            BonusSource::Base,
            None,
        ),
    ]
}

fn spell_points() -> impl IntoIterator<Item = Bonus> {
    [
        Bonus::new(
            Attribute::SpellPoints(SpellPoints::Base),
            BonusType::Stacking,
            Value::from(Attribute::SpellPoints(SpellPoints::Scaled))
                * (Value::from(Attribute::ClassLevel(PlayerClass::FavoredSoul))
                    + Value::from(Attribute::ClassLevel(PlayerClass::Sorcerer))
                    + Value::from(20f32))
                / Value::from(20f32),
            BonusSource::Base,
            None,
        ),
        Bonus::new(
            Attribute::SpellPoints(SpellPoints::Total),
            BonusType::Stacking,
            Value::from(Attribute::SpellPoints(SpellPoints::Base))
                * (Value::from(1f32) + Value::from(Attribute::SpellPoints(SpellPoints::Modifier))),
            BonusSource::Base,
            None,
        ),
    ]
}

fn spell_power_universal() -> impl IntoIterator<Item = Bonus> {
    [
        Attribute::SpellPower,
        Attribute::SpellCriticalChance,
        Attribute::SpellCriticalDamage,
    ]
    .into_iter()
    .flat_map(|attribute| {
        SpellPower::SPELL_POWERS.into_iter().map(move |sp| {
            Bonus::new(
                attribute(sp),
                BonusType::Stacking,
                Value::Attribute(attribute(SpellPower::Universal)),
                BonusSource::Base,
                None,
            )
        })
    })
}
