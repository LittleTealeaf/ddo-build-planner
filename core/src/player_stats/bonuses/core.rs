use itertools::chain;

use crate::{
    bonus::{traits::ToAttribute, Bonus, BonusType},
    types::{
        ability::Ability, alignment::Alignment, damage_type::DamageType, save::SavingThrow,
        sheltering::Sheltering, skill::Skill, spell_damage_type::SpellDamageType,
    },
};

pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
    chain![
        Ability::core_bonuses(),
        SavingThrow::core_bonuses(),
        Skill::core_bonuses(),
        Sheltering::core_bonuses(),
        universal_spell_stats(),
        skill_to_spellpower(),
    ]
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
