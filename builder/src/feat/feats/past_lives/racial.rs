use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{BonusTemplate, BonusType},
    types::{
        ability::Ability, damage_type::DamageType, race::Race, saving_throw::SavingThrow,
        sheltering::Sheltering, skill::Skill, spell_power::SpellPower, spell_school::SpellSchool,
        spell_selector::SpellSelector,
    },
};

#[allow(clippy::match_same_arms)]
pub fn racial_past_lives(race: Race, value: Decimal) -> Option<Vec<BonusTemplate>> {
    (value > Decimal::ZERO).then(|| {
        let value = value.max(Decimal::from(3));

        match race {
            Race::Dragonborn | Race::Tiefling => {
                heroic_past_lives(Skill::Spellcraft, Ability::Charisma, value)
            }
            Race::Drow => heroic_past_lives(Skill::Search, Ability::Intelligence, value),
            Race::Dwarf => heroic_past_lives(Skill::Balance, Ability::Constitution, value),
            Race::Gnome => heroic_past_lives(Skill::UseMagicalDevice, Ability::Intelligence, value),
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
                value * Decimal::from(10),
                None,
            )],
            Race::DeepGnome => vec![BonusTemplate::new(
                Attribute::SpellDC(SpellSelector::School(SpellSchool::Illusion)),
                BonusType::Stacking,
                value,
                None,
            )],
            Race::PurpleDragonKnight => vec![BonusTemplate::new(
                Sheltering::Physical,
                BonusType::Stacking,
                value * Decimal::from(3),
                None,
            )],
            Race::Razorclaw => vec![BonusTemplate::new(
                SavingThrow::Will,
                BonusType::Stacking,
                value,
                None,
            )],

            Race::Shadarkai => vec![
                // TODO: dodge
            ],
            Race::Morninglord => vec![BonusTemplate::new(
                Attribute::SpellPower(SpellPower::Damage(DamageType::Positive)),
                BonusType::Stacking,
                value * Decimal::from(3),
                None,
            )],
            Race::Trailblazer => vec![BonusTemplate::new(
                SavingThrow::Traps,
                BonusType::Stacking,
                value,
                None,
            )],
            Race::Scoundrel => vec![BonusTemplate::new(
                SavingThrow::Reflex,
                BonusType::Stacking,
                value,
                None,
            )],
        }
    })
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
