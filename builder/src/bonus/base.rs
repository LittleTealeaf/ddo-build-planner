use itertools::chain;
use rust_decimal::Decimal;

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
        sheltering::Sheltering,
        skill::Skill,
        spell_points::SpellPoints,
        spell_power::SpellPower,
    },
};

use super::{Bonus, BonusSource, BonusType, Condition, ConditionFold, Value};

const BASE: BonusSource = BonusSource::Base;

/// Returns all base bonuses that are to be included by default.
pub fn get_base_bonuses() -> impl Iterator<Item = Bonus> {
    chain!(
        ability_bonuses(),
        armor_class(),
        saving_throw(),
        spell_power_skills(),
        spell_power_universal(),
        skill(),
        health(),
        spell_points(),
        sheltering(),
        sheltering_reduction()
    )
}

fn ability_bonuses() -> impl IntoIterator<Item = Bonus> {
    Ability::ABILITIES.into_iter().flat_map(|ability| {
        [
            Bonus::new(
                Attribute::Ability(ability),
                BonusType::Stacking,
                8,
                BASE,
                None,
            ),
            Bonus::new(
                Attribute::AbilityModifier(ability),
                BonusType::Stacking,
                ((Value::Attribute(Attribute::Ability(ability)) - Value::from(10))
                    / Value::from(2))
                .floor(),
                BASE,
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
            saving_throw,
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability),
            BASE,
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
            Attribute::Skill(skill),
            BASE,
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
            Attribute::AbilityModifier(ability),
            BASE,
            None,
        )
    })
}

fn armor_class() -> impl IntoIterator<Item = Bonus> {
    [
        // Dexterity Bonus to Armor Class
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::Bonus),
            BonusType::AbilityModifier,
            Value::iter_min([
                Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity)),
                Value::condition(
                    [ArmorType::Light, ArmorType::Medium, ArmorType::Heavy]
                        .map(|armor| Condition::has(Flag::ArmorType(armor)))
                        .cond_any()
                        .unwrap(),
                    Attribute::ArmorClass(ArmorClass::ArmorMaxDex),
                    Decimal::MAX,
                ),
                Value::condition(
                    Condition::has(Attribute::from(Flag::OffHandType(OffHandType::Shield(
                        ShieldType::TowerShield,
                    )))),
                    Attribute::ArmorClass(ArmorClass::ShieldMaxDex),
                    Decimal::MAX,
                ),
            ]),
            BASE,
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
                    * (Value::Const(1.into())
                        + Value::Attribute(Attribute::ArmorClass(ArmorClass::ShieldScalar))),
                Value::Attribute(Attribute::ArmorClass(ArmorClass::ArmorBonus))
                    * (Value::Const(1.into())
                        + Value::Attribute(Attribute::ArmorClass(ArmorClass::ArmorScalar))),
                Value::from(10),
            ]
            .into_iter()
            .sum::<Value>()
                * (Value::from(1)
                    + Value::Attribute(Attribute::ArmorClass(ArmorClass::TotalScalar))),
            BASE,
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
                * (Value::from(Attribute::Health(Health::BaseModifier)) + Value::from(1)),
            BASE,
            None,
        ),
        Bonus::new(
            Attribute::Health(Health::Total),
            BonusType::Stacking,
            Value::from(Attribute::Health(Health::Bonus))
                * (Value::from(Attribute::Health(Health::Modifier)) + Value::from(1)),
            BASE,
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
                    + Value::from(20))
                / Value::from(20),
            BASE,
            None,
        ),
        Bonus::new(
            Attribute::SpellPoints(SpellPoints::Total),
            BonusType::Stacking,
            Value::from(Attribute::SpellPoints(SpellPoints::Base))
                * (Value::from(1) + Value::from(Attribute::SpellPoints(SpellPoints::Modifier))),
            BASE,
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
                attribute(SpellPower::Universal),
                BASE,
                None,
            )
        })
    })
}

fn sheltering() -> impl IntoIterator<Item = Bonus> {
    [
        Bonus::new(
            Sheltering::MagicalCap,
            BonusType::Stacking,
            Value::condition(
                Condition::has(Flag::from(ArmorType::Medium))
                    | Condition::has(Flag::from(ArmorType::Heavy)),
                Attribute::Sheltering(Sheltering::Magical),
                Value::condition(Condition::has(Flag::from(ArmorType::Light)), 100, 50),
            ),
            BASE,
            None,
        ),
        Bonus::new(
            Sheltering::MagicalTotal,
            BonusType::Stacking,
            Value::from(Attribute::Sheltering(Sheltering::Magical))
                .min(Value::from(Attribute::Sheltering(Sheltering::MagicalCap))),
            BASE,
            None,
        ),
        Bonus::new(
            Sheltering::PhysicalTotal,
            BonusType::Stacking,
            Attribute::Sheltering(Sheltering::Physical),
            BASE,
            None,
        ),
    ]
}

fn sheltering_reduction() -> impl IntoIterator<Item = Bonus> {
    [
        (Sheltering::PhysicalTotal, Sheltering::PhysicalReduction),
        (Sheltering::MagicalTotal, Sheltering::MagicalReduction),
    ]
    .into_iter()
    .map(|(total, reduction)| {
        Bonus::new(
            reduction,
            BonusType::Stacking,
            Value::from(100)
                * (Value::from(1)
                    - (Value::from(100)
                        / (Value::from(100) + Value::from(Attribute::Sheltering(total))))),
            BASE,
            None,
        )
    })
}
