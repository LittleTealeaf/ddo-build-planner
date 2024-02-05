use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{BonusTemplate, BonusType, ToValue, Value},
    feat::ToFeat,
    types::{
        ability::Ability, damage_type::DamageType, race::Race, skill::Skill,
        spell_power::SpellPower,
    },
};

use super::PastLife;

pub fn racial_past_lives(race: Race, value: Decimal) -> Vec<BonusTemplate> {
    match race {
        Race::Dragonborn | Race::Tiefling => {
            heroic_past_lives(Skill::Spellcraft, Ability::Charisma, value)
        }
        Race::Drow => heroic_past_lives(Skill::Search, Ability::Intelligence, value),
        Race::Dwarf => heroic_past_lives(Skill::Balance, Ability::Constitution, value),
        Race::DeepGnome => heroic_past_lives(Skill::UseMagicalDevice, Ability::Intelligence, value),
        Race::Halfling => heroic_past_lives(Skill::MoveSilently, Ability::Dexterity, value),
        Race::HalfElf => heroic_past_lives(Skill::Diplomacy, Ability::Charisma, value),
        Race::HalfOrc => heroic_past_lives(Skill::Intimidate, Ability::Strength, value),
        Race::Human => heroic_past_lives(Skill::Haggle, Ability::Wisdom, value),
        Race::Warforged => heroic_past_lives(Skill::Repair, Ability::Constitution, value),
        Race::Aasimar => heroic_past_lives(Skill::Heal, Ability::Wisdom, value),
        Race::Tabaxi => heroic_past_lives(Skill::Tumble, Ability::Dexterity, value),
        Race::Shifter | Race::WoodElf | Race::Elf => {
            heroic_past_lives(Skill::Spot, Ability::Dexterity, value)
        }
        Race::Scourge => vec![
            // TODO: doublestrike
        ],
        Race::Bladeforged => vec![BonusTemplate::new(
            Attribute::SpellPower(SpellPower::Damage(DamageType::Repair)),
            BonusType::Stacking,
            Value::max(
                3.to_value(),
                PastLife::Racial(Race::Warforged).to_feat().to_value(),
            ),
            None,
        )],
        _ => todo!(),
    }
}

fn heroic_past_lives(skill: Skill, ability: Ability, value: Decimal) -> Vec<BonusTemplate> {
    [
        (value >= Decimal::ONE).then(|| BonusTemplate::new(skill, BonusType::Stacking, 1, None)),
        (value >= Decimal::TWO).then(|| BonusTemplate::new(ability, BonusType::Stacking, 1, None)),
        // TODO: Bonus racial action point
    ]
    .into_iter()
    .flatten()
    .collect()
}
