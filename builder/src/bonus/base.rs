use itertools::chain;

use crate::{
    attribute::Attribute,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        damage_type::DamageType,
        flag::OffHandType,
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
        sheltering_reduction(),
        armor_check_penalties()
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
                ((Value::Attribute(Attribute::Ability(ability)) - Value::TEN) / Value::TWO).floor(),
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
            skill,
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
            ArmorClass::Bonus,
            BonusType::AbilityModifier,
            Value::iter_min([
                Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity)),
                Value::condition(
                    [ArmorType::Light, ArmorType::Medium, ArmorType::Heavy]
                        .map(Condition::has)
                        .cond_any()
                        .unwrap(),
                    ArmorClass::ArmorMaxDex,
                    Value::MAX,
                ),
                Value::condition(
                    Condition::has(OffHandType::Shield(ShieldType::TowerShield)),
                    ArmorClass::ShieldMaxDex,
                    Value::MAX,
                ),
            ]),
            BASE,
            None,
        ),
        // Total Armor Class Bonus
        Bonus::new(
            ArmorClass::TotalArmorClass,
            BonusType::Standard,
            [
                Value::from(ArmorClass::Bonus),
                Value::from(ArmorClass::NaturalArmor),
                Value::from(ArmorClass::ShieldBonus)
                    * (Value::ONE + Value::from(ArmorClass::ShieldScalar)),
                Value::from(ArmorClass::ArmorBonus)
                    * (Value::ONE + Value::from(ArmorClass::ArmorScalar)),
                Value::TEN,
            ]
            .into_iter()
            .sum::<Value>()
                * (Value::ONE + Value::from(ArmorClass::TotalScalar)),
            BASE,
            None,
        ),
    ]
}

fn health() -> impl IntoIterator<Item = Bonus> {
    [
        Bonus::new(
            Health::Bonus,
            BonusType::Stacking,
            Value::from(Health::Base) * (Value::from(Health::BaseModifier) + Value::ONE),
            BASE,
            None,
        ),
        Bonus::new(
            Health::Total,
            BonusType::Stacking,
            Value::from(Health::Bonus) * (Value::from(Health::Modifier) + Value::ONE),
            BASE,
            None,
        ),
    ]
}

fn spell_points() -> impl IntoIterator<Item = Bonus> {
    [
        Bonus::new(
            SpellPoints::Base,
            BonusType::Stacking,
            Value::from(SpellPoints::Scaled)
                * (Value::from(PlayerClass::FavoredSoul)
                    + Value::from(PlayerClass::Sorcerer)
                    + Value::from(20))
                / Value::from(20),
            BASE,
            None,
        ),
        Bonus::new(
            SpellPoints::Total,
            BonusType::Stacking,
            Value::from(SpellPoints::Base) * (Value::ONE + Value::from(SpellPoints::Modifier)),
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
                Condition::has(ArmorType::Medium) | Condition::has(ArmorType::Heavy),
                Sheltering::Magical,
                Value::condition(Condition::has(ArmorType::Light), 100, 50),
            ),
            BASE,
            None,
        ),
        Bonus::new(
            Sheltering::MagicalTotal,
            BonusType::Stacking,
            Value::from(Sheltering::Magical).min(Value::from(Sheltering::MagicalCap)),
            BASE,
            None,
        ),
        Bonus::new(
            Sheltering::PhysicalTotal,
            BonusType::Stacking,
            Sheltering::Physical,
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
            Value::ONE_HUNDRED
                * (Value::ONE - (Value::ONE_HUNDRED / (Value::ONE_HUNDRED + Value::from(total)))),
            BASE,
            None,
        )
    })
}

fn armor_check_penalties() -> impl Iterator<Item = Bonus> {
    [
        (Skill::Balance, 1),
        (Skill::Hide, 1),
        (Skill::Jump, 1),
        (Skill::MoveSilently, 1),
        (Skill::Swim, 2),
        (Skill::Tumble, 1),
    ]
    .into_iter()
    .map(|(skill, scale)| {
        Bonus::new(
            skill,
            BonusType::Stacking,
            Value::from(-scale) * Value::from(Attribute::ArmorCheckPenalty),
            BASE,
            Condition::has(Attribute::ArmorCheckPenalty),
        )
    })
}
