use itertools::chain;

use crate::{
    attribute::Attribute,
    bonus::{traits::ToValue, Bonus, BonusType},
    traits::IterValues,
    types::{ability::Ability, skill::Skill, spell_power::SpellPower},
    val,
};

pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
    chain![
        ability_modfiiers(),
        skill_bonuses(),
        universal_spell_power(),
        spellcraft_to_spellpower(),
    ]
}

fn ability_modfiiers() -> impl Iterator<Item = Bonus> {
    Ability::values().map(|ability| {
        Bonus::new(
            ability.modifier(),
            (ability.score().to_value() - val!(10)) / val!(2),
            BonusType::Ability,
            ability.score(),
        )
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

fn spellcraft_to_spellpower() -> impl Iterator<Item = Bonus> {
    SpellPower::SPELL_POWERS.into_iter().map(|power| {
        Bonus::new(
            Attribute::SpellPower(power),
            Attribute::Skill(Skill::Spellcraft),
            BonusType::Stacking,
            Attribute::Skill(Skill::Spellcraft),
        )
    })
}

fn universal_spell_power() -> impl Iterator<Item = Bonus> {
    SpellPower::SPELL_POWERS.into_iter().map(|power| {
        Bonus::new(
            Attribute::SpellPower(power),
            Attribute::SpellPower(SpellPower::Universal),
            BonusType::Stacking,
            Attribute::SpellPower(SpellPower::Universal),
        )
    })
}
