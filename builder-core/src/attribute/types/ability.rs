use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

use super::Skill;

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability {
    pub const ALL: [Ability; 6] = [
        Ability::Strength,
        Ability::Dexterity,
        Ability::Constitution,
        Ability::Intelligence,
        Ability::Wisdom,
        Ability::Charisma,
    ];

    pub fn get_score_bonuses(&self, value: f32) -> Vec<Bonus> {
        vec![Bonus::new(
            Attribute::AbilityModifier(*self),
            BonusType::AbilityModifier,
            ((value - 10f32) / 2f32).floor().into(),
            Attribute::Ability(*self).into(),
            None,
        )]
    }

    pub fn get_modifier_bonuses(&self, value: f32) -> Vec<Bonus> {
        let mut vec = match self {
            Ability::Strength => vec![
                modifier_skill_bonus(Ability::Strength, Skill::Jump, value),
                modifier_skill_bonus(Ability::Strength, Skill::Swim, value),
            ],
            Ability::Dexterity => vec![
                modifier_skill_bonus(Ability::Dexterity, Skill::Balance, value),
                modifier_skill_bonus(Ability::Dexterity, Skill::Hide, value),
                modifier_skill_bonus(Ability::Dexterity, Skill::MoveSilently, value),
                modifier_skill_bonus(Ability::Dexterity, Skill::OpenLock, value),
                modifier_skill_bonus(Ability::Dexterity, Skill::Tumble, value),
            ],
            Ability::Constitution => vec![modifier_skill_bonus(
                Ability::Constitution,
                Skill::Concentration,
                value,
            )],
            Ability::Intelligence => vec![
                modifier_skill_bonus(Ability::Intelligence, Skill::DisableDevice, value),
                modifier_skill_bonus(Ability::Intelligence, Skill::Repair, value),
                modifier_skill_bonus(Ability::Intelligence, Skill::Search, value),
                modifier_skill_bonus(Ability::Intelligence, Skill::Spellcraft, value),
            ],
            Ability::Wisdom => vec![
                modifier_skill_bonus(Ability::Wisdom, Skill::Heal, value),
                modifier_skill_bonus(Ability::Wisdom, Skill::Listen, value),
                modifier_skill_bonus(Ability::Wisdom, Skill::Spot, value),
            ],
            Ability::Charisma => vec![
                modifier_skill_bonus(Ability::Charisma, Skill::Bluff, value),
                modifier_skill_bonus(Ability::Charisma, Skill::Diplomacy, value),
                modifier_skill_bonus(Ability::Charisma, Skill::Haggle, value),
                modifier_skill_bonus(Ability::Charisma, Skill::Intimidate, value),
                modifier_skill_bonus(Ability::Charisma, Skill::Perform, value),
                modifier_skill_bonus(Ability::Charisma, Skill::UseMagicalDevice, value),
            ],
        };

        return vec;
    }
}

fn modifier_skill_bonus(ability: Ability, skill: Skill, value: f32) -> Bonus {
    Bonus::new(
        skill.into(),
        BonusType::AbilityModifier,
        value.into(),
        Attribute::AbilityModifier(ability).into(),
        None,
    )
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Ability::Strength => "Strength",
                Ability::Dexterity => "Dexterity",
                Ability::Constitution => "Constitution",
                Ability::Intelligence => "Intelligence",
                Ability::Wisdom => "Wisdom",
                Ability::Charisma => "Charisma",
            }
        )
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Self {
        Attribute::Skill(value)
    }
}
