use itertools::chain;

use crate::{
    attribute::Attribute,
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusSource, BonusType,
    },
    traits::IterValues,
    types::{
        ability::Ability, alignment::Alignment, damage_type::DamageType, save::SavingThrow,
        sheltering::Sheltering, skill::Skill, spell_damage_type::SpellDamageType,
    },
    val,
};

pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
    chain![
        abilities(),
        armor_check_penalty(),
        skill_bonuses(),
        universal_spell_stats(),
        skill_to_spellpower(),
        saving_throws(),
        sheltering(),
    ]
}

fn abilities() -> impl Iterator<Item = Bonus> {
    Ability::values().flat_map(|ability| {
        [
            Bonus::new(
                ability.modifier(),
                (ability.score().to_value().max(val!(0)) / val!(2)).floor() - val!(5),
                BonusType::Ability,
                ability.score(),
            ),
            Bonus::new(
                ability.score(),
                val!(8),
                BonusType::Stacking,
                ability.score(),
            ),
        ]
    })
}

fn saving_throws() -> impl Iterator<Item = Bonus> {
    SavingThrow::values().filter_map(|save| {
        Some(Bonus::new(
            save,
            save.ability()?.modifier(),
            BonusType::Ability,
            save.ability()?.modifier(),
        ))
    })
}

fn skill_bonuses() -> impl Iterator<Item = Bonus> {
    Skill::values().map(|skill| {
        Bonus::new(
            skill,
            skill.ability().modifier(),
            BonusType::Ability,
            skill.ability().modifier(),
        )
    })
}

fn armor_check_penalty() -> impl Iterator<Item = Bonus> {
    [
        (Skill::Balance, val!(-1)),
        (Skill::Hide, val!(-1)),
        (Skill::Jump, val!(-1)),
        (Skill::MoveSilently, val!(-1)),
        (Skill::Swim, val!(-2)),
        (Skill::Tumble, val!(-1)),
    ]
    .into_iter()
    .map(|(skill, scale)| {
        Bonus::new(
            skill,
            scale * Attribute::ArmorCheckPenalty.to_value(),
            BonusType::Stacking,
            Attribute::ArmorCheckPenalty,
        )
    })
}

fn skill_to_spellpower() -> impl Iterator<Item = Bonus> {
    fn spellpower_bonus(
        damages: impl IntoIterator<Item = DamageType>,
        skill: Skill,
    ) -> impl Iterator<Item = Bonus> {
        damages.into_iter().map(move |damage| {
            Bonus::new(
                SpellDamageType::from(damage).spell_power(),
                skill.attribute(),
                BonusType::Stacking,
                skill.attribute(),
            )
        })
    }

    chain!(
        spellpower_bonus(
            [
                DamageType::Acid,
                DamageType::Cold,
                DamageType::Electric,
                DamageType::Fire,
                DamageType::Force,
                DamageType::Poison,
                DamageType::Physical,
                DamageType::Untyped,
                DamageType::Light,
                DamageType::Aligned(Alignment::Lawful),
                DamageType::Aligned(Alignment::Evil),
                DamageType::Aligned(Alignment::Chaotic),
                DamageType::Aligned(Alignment::Good),
            ],
            Skill::Spellcraft
        ),
        spellpower_bonus([DamageType::Positive, DamageType::Negative], Skill::Heal),
        spellpower_bonus([DamageType::Rust, DamageType::Repair], Skill::Repair),
    )
}

fn universal_spell_stats() -> impl Iterator<Item = Bonus> {
    SpellDamageType::SPELL_POWERS.into_iter().flat_map(|power| {
        [
            Bonus::new(
                power.spell_power(),
                SpellDamageType::Universal.spell_power(),
                BonusType::Stacking,
                SpellDamageType::Universal.spell_power(),
            ),
            Bonus::new(
                power.spell_critical_chance(),
                SpellDamageType::Universal.spell_critical_chance(),
                BonusType::Stacking,
                SpellDamageType::Universal.spell_critical_chance(),
            ),
            Bonus::new(
                power.spell_critical_damage(),
                SpellDamageType::Universal.spell_critical_damage(),
                BonusType::Stacking,
                SpellDamageType::Universal.spell_critical_damage(),
            ),
        ]
    })
}

fn sheltering() -> impl IntoIterator<Item = Bonus> {
    [
        Bonus::new(
            Sheltering::MagicalReduction,
            val!(1)
                - (val!(100)
                    / (val!(100)
                        + (Sheltering::Magical
                            .attribute()
                            .to_value()
                            .min(Sheltering::MagicalCap.attribute().to_value())
                            + Sheltering::MagicalUncapped.attribute().to_value()))),
            BonusType::Stacking,
            BonusSource::Custom("Sheltering".to_owned()),
        ),
        Bonus::new(
            Sheltering::PhysicalReduction,
            val!(1) - (val!(100) / (val!(100) + Sheltering::Physical.attribute().to_value())),
            BonusType::Stacking,
            BonusSource::Custom("Sheltering".to_owned()),
        ),
    ]
}
