use itertools::chain;

use crate::{
    attribute::Attribute,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        damage_type::DamageType,
        flag::OffHandType,
        health::Health,
        item_type::{ArmorType, ShieldType},
        player_class::PlayerClass,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        spell_points::SpellPoints,
        spell_power::SpellPower,
    },
};

use super::{
    Bonus, BonusSource, BonusTemplate, BonusType, Condition, ConditionFold, ToValue, Value,
};

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
    .map(|bonus| bonus.to_bonus(BonusSource::Base))
}

fn ability_bonuses() -> impl IntoIterator<Item = BonusTemplate> {
    Ability::ABILITIES.into_iter().flat_map(|ability| {
        [
            BonusTemplate::new(Attribute::Ability(ability), BonusType::Stacking, 8, None),
            BonusTemplate::new(
                Attribute::AbilityModifier(ability),
                BonusType::Stacking,
                ((Value::Attribute(Attribute::Ability(ability)) - Value::TEN) / Value::TWO).floor(),
                None,
            ),
        ]
    })
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
        BonusTemplate::new(
            skill,
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability),
            None,
        )
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
            ArmorClass::TotalArmorClass,
            BonusType::Standard,
            Value::iter_sum([
                ArmorClass::Bonus.value(),
                ArmorClass::NaturalArmor.value(),
                ArmorClass::ShieldBonus.value() * (Value::ONE + ArmorClass::ShieldScalar.value()),
                ArmorClass::ArmorBonus.value() * (Value::ONE + ArmorClass::ArmorScalar.value()),
                Value::TEN,
            ]) * (Value::ONE + ArmorClass::TotalScalar.value()),
            None,
        ),
    ]
}

fn health() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            Health::Bonus,
            BonusType::Stacking,
            Health::Base.value() * (Health::BaseModifier.value() + Value::ONE),
            None,
        ),
        BonusTemplate::new(
            Health::Total,
            BonusType::Stacking,
            Health::Bonus.value() * (Health::Modifier.value() + Value::ONE),
            None,
        ),
    ]
}

fn spell_points() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            SpellPoints::Base,
            BonusType::Stacking,
            SpellPoints::Scaled.value()
                * (PlayerClass::FavoredSoul.value() + PlayerClass::Sorcerer.value() + 20.value())
                / 20.value(),
            None,
        ),
        BonusTemplate::new(
            SpellPoints::Total,
            BonusType::Stacking,
            SpellPoints::Base.value() * (Value::ONE + SpellPoints::Modifier.value()),
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
                Value::condition(Condition::has(ArmorType::Light), 100, 50),
            ),
            None,
        ),
        BonusTemplate::new(
            Sheltering::MagicalTotal,
            BonusType::Stacking,
            Sheltering::Magical
                .value()
                .min(Sheltering::MagicalCap.value()),
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
                * (Value::ONE - (Value::ONE_HUNDRED / (Value::ONE_HUNDRED + total.value()))),
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
            (-scale).value() * Attribute::ArmorCheckPenalty.value(),
            Condition::has(Attribute::ArmorCheckPenalty),
        )
    })
}
