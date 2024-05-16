use core::iter::once;
use itertools::chain;
use utils::{enums::StaticOptions, hashmap::IntoGroupedHashMap};

use crate::{
    attribute::Attribute,
    bonus::{
        Bonus, BonusSource, BonusTemplate, BonusType, Condition, ConditionFold, ToValue, Value,
    },
    feat::{HeroicPastLife, PastLifeFeat, RacialPastLife},
    types::{
        ability::Ability,
        absorption::{Absorption, AbsorptionSource},
        armor_class::ArmorClass,
        damage_type::DamageType,
        flag::{Flag, MainHandType, OffHandType},
        health::Health,
        item_type::{ArmorType, ShieldType, WeaponType},
        player_class::PlayerClass,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        spell_points::SpellPoints,
        spell_power::SpellPower,
    },
    val,
};

/// Returns all base bonuses that are to be included by default.
pub fn get_base_bonuses() -> impl Iterator<Item = Bonus> {
    chain!(
        ability_bonuses(),
        armor_class(),
        saving_throw(),
        secondary_saves(),
        spell_power_skills(),
        spell_power_universal(),
        skill(),
        health(),
        spell_points(),
        sheltering(),
        sheltering_reduction(),
        armor_check_penalties(),
        absorption(),
        completionist_feats(),
        two_handed_fighting(),
    )
    .map(|bonus| bonus.to_bonus(BonusSource::Base))
}

fn ability_bonuses() -> impl IntoIterator<Item = BonusTemplate> {
    Ability::ABILITIES
        .into_iter()
        .map(|ability| {
            BonusTemplate::new(
                Attribute::AbilityModifier(ability),
                BonusType::Stacking,
                ((Value::Attribute(Attribute::Ability(ability)) - val!(10)) / val!(2)).floor(),
                None,
            )
        })
        .chain(once(BonusTemplate::new(
            Attribute::Ability(Ability::All),
            BonusType::Stacking,
            val!(8),
            None,
        )))
}

fn saving_throw() -> impl IntoIterator<Item = BonusTemplate> {
    [
        (Ability::Dexterity, SavingThrow::Reflex),
        (Ability::Constitution, SavingThrow::Fortitude),
        (Ability::Wisdom, SavingThrow::Will),
    ]
    .into_iter()
    .map(|(ability, saving_throw)| {
        BonusTemplate::new(
            saving_throw,
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability),
            None,
        )
    })
}

fn secondary_saves() -> impl Iterator<Item = BonusTemplate> {
    SavingThrow::SECONDARY.into_iter().filter_map(|skill| {
        Some(BonusTemplate::new(
            skill,
            BonusType::Stacking,
            skill.get_parent()?.to_value(),
            None,
        ))
    })
}

fn spell_power_skills() -> impl IntoIterator<Item = BonusTemplate> {
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
        BonusTemplate::new(
            Attribute::SpellPower(SpellPower::Damage(damage_type)),
            BonusType::Stacking,
            Attribute::Skill(skill),
            None,
        )
    })
}

fn skill() -> impl IntoIterator<Item = BonusTemplate> {
    Skill::SKILLS.into_iter().filter_map(|skill| {
        Some(BonusTemplate::new(
            skill,
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(skill.get_ability()?),
            None,
        ))
    })
}

fn armor_class() -> impl IntoIterator<Item = BonusTemplate> {
    [
        // Dexterity Bonus to Armor Class
        BonusTemplate::new(
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
            None,
        ),
        // Total Armor Class Bonus
        BonusTemplate::new(
            ArmorClass::Total,
            BonusType::Standard,
            Value::iter_sum([
                ArmorClass::Bonus.to_value(),
                ArmorClass::NaturalArmor.to_value(),
                ArmorClass::ShieldBonus.to_value()
                    * (Value::ONE + ArmorClass::ShieldScalar.to_value()),
                ArmorClass::ArmorBonus.to_value()
                    * (Value::ONE + ArmorClass::ArmorScalar.to_value()),
                Value::TEN,
            ]) * (Value::ONE + ArmorClass::TotalScalar.to_value()),
            None,
        ),
    ]
}

fn health() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            Health::Bonus,
            BonusType::Stacking,
            Health::Base.to_value() * (Health::BaseModifier.to_value() + Value::ONE),
            None,
        ),
        BonusTemplate::new(
            Health::Total,
            BonusType::Stacking,
            Health::Bonus.to_value() * (Health::Modifier.to_value() + Value::ONE),
            None,
        ),
    ]
}

fn spell_points() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            SpellPoints::Base,
            BonusType::Stacking,
            SpellPoints::Scaled.to_value()
                * (PlayerClass::FavoredSoul.to_value()
                    + PlayerClass::Sorcerer.to_value()
                    + val!(20))
                / val!(20),
            None,
        ),
        BonusTemplate::new(
            SpellPoints::Total,
            BonusType::Stacking,
            SpellPoints::Base.to_value() * (Value::ONE + SpellPoints::Modifier.to_value()),
            None,
        ),
    ]
}

fn spell_power_universal() -> impl IntoIterator<Item = BonusTemplate> {
    [
        Attribute::SpellPower,
        Attribute::SpellCriticalChance,
        Attribute::SpellCriticalDamage,
    ]
    .into_iter()
    .flat_map(|attribute| {
        SpellPower::SPELL_POWERS.into_iter().map(move |sp| {
            BonusTemplate::new(
                attribute(sp),
                BonusType::Stacking,
                attribute(SpellPower::Universal),
                None,
            )
        })
    })
}

fn sheltering() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            Sheltering::MagicalCap,
            BonusType::Stacking,
            Value::condition(
                Condition::has(ArmorType::Medium) | Condition::has(ArmorType::Heavy),
                Sheltering::Magical,
                Value::condition(Condition::has(ArmorType::Light), val!(100), val!(50)),
            ),
            None,
        ),
        BonusTemplate::new(
            Sheltering::MagicalTotal,
            BonusType::Stacking,
            Sheltering::Magical
                .to_value()
                .min(Sheltering::MagicalCap.to_value()),
            None,
        ),
        BonusTemplate::new(
            Sheltering::PhysicalTotal,
            BonusType::Stacking,
            Sheltering::Physical,
            None,
        ),
    ]
}

fn sheltering_reduction() -> impl IntoIterator<Item = BonusTemplate> {
    [
        (Sheltering::PhysicalTotal, Sheltering::PhysicalReduction),
        (Sheltering::MagicalTotal, Sheltering::MagicalReduction),
    ]
    .into_iter()
    .map(|(total, reduction)| {
        BonusTemplate::new(
            reduction,
            BonusType::Stacking,
            Value::ONE_HUNDRED
                * (Value::ONE - (Value::ONE_HUNDRED / (Value::ONE_HUNDRED + total.to_value()))),
            None,
        )
    })
}

fn armor_check_penalties() -> impl Iterator<Item = BonusTemplate> {
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
        BonusTemplate::new(
            skill,
            BonusType::Stacking,
            (-scale).to_value() * Attribute::ArmorCheckPenalty.to_value(),
            Condition::has(Attribute::ArmorCheckPenalty),
        )
    })
}

fn absorption() -> impl Iterator<Item = BonusTemplate> {
    DamageType::get_static().map(|damage_type| {
        BonusTemplate::new(
            Absorption::Total(damage_type),
            BonusType::Stacking,
            Value::ONE
                - Value::iter_product(AbsorptionSource::get_static().map(|bonus_type| {
                    Value::ONE - Absorption::Bonus(damage_type, bonus_type).to_value()
                })),
            None,
        )
    })
}

fn completionist_feats() -> impl IntoIterator<Item = BonusTemplate> {
    [
        {
            // HEROIC COMPLETIONIST
            let condition = PlayerClass::get_static()
                .map(|class| (class.get_parent_class().unwrap_or(class), class))
                .into_grouped_hash_map()
                .into_values()
                .map(|set| {
                    set.into_iter()
                        .map(|class| HeroicPastLife(class).to_value())
                        .sum::<Value>()
                        .greater_than(Value::ZERO)
                })
                .cond_all();
            BonusTemplate::feat(PastLifeFeat::HeroicCompletionist, condition)
        },
        {
            // RACIAL COMPLETIONIST
            let condition = RacialPastLife::RACES
                .map(|race| (race.get_base().unwrap_or(race), race))
                .into_grouped_hash_map()
                .into_values()
                .map(|set| {
                    set.into_iter()
                        .map(ToValue::to_value)
                        .sum::<Value>()
                        .greater_or_equal_to(val!(3))
                })
                .cond_all();

            BonusTemplate::feat(PastLifeFeat::RacialCompletionist, condition)
        },
    ]
}

// TODO: convert this to the other method (flag to flag)
fn two_handed_fighting() -> impl Iterator<Item = BonusTemplate> {
    once(BonusTemplate::flag(
        Flag::IsTwoHandedFighting,
        WeaponType::TWO_HANDED_MELEE_WEAPONS
            .map(|weapon| Condition::has(MainHandType::Weapon(weapon)))
            .cond_any(),
    ))
}
