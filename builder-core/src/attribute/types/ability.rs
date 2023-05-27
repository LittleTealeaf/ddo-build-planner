use std::fmt::Display;

use enum_map::Enum;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType, CloneBonus},
};

use super::{SavingThrow, Skill};

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    /// All values
    All,
}

impl Ability {
    pub const VALUES: [Ability; 6] = [
        Ability::Strength,
        Ability::Dexterity,
        Ability::Constitution,
        Ability::Intelligence,
        Ability::Wisdom,
        Ability::Charisma,
    ];

    fn modifier_bonus<T>(&self, attribute: T, value: f32) -> Bonus
    where
        Attribute: From<T>,
    {
        Bonus::new(
            attribute.into(),
            BonusType::AbilityModifier,
            value.into(),
            Attribute::AbilityModifier(*self).into(),
            None,
        )
    }
}

pub struct _AbilityScore;

impl GetBonuses<_AbilityScore> for Ability {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (!matches!(self, Self::All)).then(|| {
            vec![Bonus::new(
                Attribute::AbilityModifier(*self),
                BonusType::Stacking,
                ((value - 10f32) / 2f32).floor().into(),
                Attribute::Ability(*self).into(),
                None,
            )]
        })
    }
}

pub struct _AbilityModifier;

impl GetBonuses<_AbilityModifier> for Ability {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Ability::Strength => Some(vec![
                self.modifier_bonus(Skill::Jump, value),
                self.modifier_bonus(Skill::Swim, value),
            ]),
            Ability::Dexterity => Some(vec![
                self.modifier_bonus(Skill::Balance, value),
                self.modifier_bonus(Skill::Hide, value),
                self.modifier_bonus(Skill::MoveSilently, value),
                self.modifier_bonus(Skill::OpenLock, value),
                self.modifier_bonus(Skill::Tumble, value),
                self.modifier_bonus(SavingThrow::Reflex, value),
            ]),
            Ability::Constitution => Some(vec![
                self.modifier_bonus(Skill::Concentration, value),
                self.modifier_bonus(SavingThrow::Fortitude, value),
            ]),
            Ability::Intelligence => Some(vec![
                self.modifier_bonus(Skill::DisableDevice, value),
                self.modifier_bonus(Skill::Repair, value),
                self.modifier_bonus(Skill::Search, value),
                self.modifier_bonus(Skill::Spellcraft, value),
            ]),
            Ability::Wisdom => Some(vec![
                self.modifier_bonus(Skill::Heal, value),
                self.modifier_bonus(Skill::Listen, value),
                self.modifier_bonus(Skill::Spot, value),
                self.modifier_bonus(SavingThrow::Will, value),
            ]),
            Ability::Charisma => Some(vec![
                self.modifier_bonus(Skill::Bluff, value),
                self.modifier_bonus(Skill::Diplomacy, value),
                self.modifier_bonus(Skill::Haggle, value),
                self.modifier_bonus(Skill::Intimidate, value),
                self.modifier_bonus(Skill::Perform, value),
                self.modifier_bonus(Skill::UseMagicalDevice, value),
            ]),
            Ability::All => None,
        }
    }
}

impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::VALUES
                .map(|ability| {
                    Bonus::new(
                        ability.into(),
                        bonus.get_type(),
                        bonus.get_value(),
                        bonus.get_source(),
                        bonus.get_condition(),
                    )
                })
                .to_vec()
        })
    }
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ability::Strength => write!(f, "Strength"),
            Ability::Dexterity => write!(f, "Dexterity"),
            Ability::Constitution => write!(f, "Constitution"),
            Ability::Intelligence => write!(f, "Intelligence"),
            Ability::Wisdom => write!(f, "Wisdom"),
            Ability::Charisma => write!(f, "Charisma"),
            Ability::All => write!(f, "All"),
        }
    }
}

impl From<Ability> for Attribute {
    fn from(value: Ability) -> Self {
        Attribute::Ability(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_attribute_bonuses() {
        for ability in Ability::VALUES {
            let bonuses = Attribute::Ability(ability)
                .get_bonuses(20f32)
                .expect("Expected Bonuses to be returned for an Ability Score");

            assert!(bonuses.len() >= 1);
        }
    }

    #[test]
    fn modifier_attribute_gets_bonuses() {
        for ability in Ability::VALUES {
            let bonuses = Attribute::AbilityModifier(ability).get_bonuses(20f32);

            assert!(
                !matches!(bonuses, None),
                "No bonuses returned for {} ability modifier",
                ability
            );
        }
    }
}
