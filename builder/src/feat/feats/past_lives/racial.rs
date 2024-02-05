use rust_decimal::Decimal;

use crate::{
    bonus::{BonusTemplate, BonusType},
    types::{ability::Ability, race::Race, skill::Skill},
};

pub fn racial_past_lives(race: Race, value: Decimal) -> Option<Vec<BonusTemplate>> {
    let (skill, ability) = match race {
        Race::Dragonborn | Race::Tiefling => Some((Skill::Spellcraft, Ability::Charisma)),
        Race::Drow => Some((Skill::Search, Ability::Intelligence)),
        Race::Dwarf => Some((Skill::Balance, Ability::Constitution)),
        Race::Gnome => Some((Skill::UseMagicalDevice, Ability::Intelligence)),
        Race::Halfling => Some((Skill::MoveSilently, Ability::Dexterity)),
        Race::HalfElf => Some((Skill::Diplomacy, Ability::Charisma)),
        Race::HalfOrc => Some((Skill::Intimidate, Ability::Strength)),
        Race::Human => Some((Skill::Haggle, Ability::Wisdom)),
        Race::Warforged => Some((Skill::Repair, Ability::Constitution)),
        Race::Aasimar => Some((Skill::Heal, Ability::Wisdom)),
        Race::Shifter | Race::Elf | Race::WoodElf => Some((Skill::Spot, Ability::Dexterity)),
        Race::Tabaxi => Some((Skill::Tumble, Ability::Dexterity)),
        _ => None,
    }?;

    Some(
        [
            (value >= Decimal::ONE)
                .then(|| BonusTemplate::new(skill, BonusType::Stacking, 1, None)),
            (value >= Decimal::TWO)
                .then(|| BonusTemplate::new(ability, BonusType::Stacking, 1, None)),
            // TODO: Bonus Racial Action Point
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}
