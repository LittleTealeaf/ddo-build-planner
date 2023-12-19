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
        spell_power(),
        skill(),
        health(),
        spell_points()
    )
}

fn ability_bonuses() -> impl Iterator<Item = Bonus> {
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

fn saving_throw() -> impl Iterator<Item = Bonus> {
    macro_rules! map {
        ($ability: ident, $save: ident) => {
            (Ability::$ability, SavingThrow::$save)
        };
    }

    [
        map!(Dexterity, Reflex),
        map!(Constitution, Fortitude),
        map!(Wisdom, Will),
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

fn spell_power() -> impl Iterator<Item = Bonus> {
    macro_rules! map {
        ($skill: ident, $damage_type: ident) => {
            (Skill::$skill, DamageType::$damage_type)
        };
    }

    [
        map!(Heal, Positive),
        map!(Heal, Negative),
        map!(Perform, Sonic),
        map!(Spellcraft, Acid),
        map!(Spellcraft, Cold),
        map!(Spellcraft, Electric),
        map!(Spellcraft, Fire),
        map!(Spellcraft, Force),
        map!(Spellcraft, Light),
        map!(Spellcraft, Poison),
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

fn skill() -> impl Iterator<Item = Bonus> {
    macro_rules! map {
        ($ability: ident, $skill: ident) => {
            (Ability::$ability, Skill::$skill)
        };
    }

    [
        map!(Dexterity, Balance),
        map!(Charisma, Bluff),
        map!(Constitution, Concentration),
        map!(Charisma, Diplomacy),
        map!(Intelligence, DisableDevice),
        map!(Charisma, Haggle),
        map!(Wisdom, Heal),
        map!(Dexterity, Hide),
        map!(Charisma, Intimidate),
        map!(Strength, Jump),
        map!(Wisdom, Listen),
        map!(Dexterity, MoveSilently),
        map!(Dexterity, OpenLock),
        map!(Charisma, Perform),
        map!(Intelligence, Repair),
        map!(Intelligence, Search),
        map!(Intelligence, Spellcraft),
        map!(Wisdom, Spot),
        map!(Strength, Swim),
        map!(Dexterity, Tumble),
        map!(Charisma, UseMagicalDevice),
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

fn armor_class() -> impl Iterator<Item = Bonus> {
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
    .into_iter()
}

fn health() -> impl Iterator<Item = Bonus> {
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
    .into_iter()
}

fn spell_points() -> impl Iterator<Item = Bonus> {
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
    .into_iter()
}
