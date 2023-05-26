use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusType},
};

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
        vec![]
    }
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
