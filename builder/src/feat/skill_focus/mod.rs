use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
    types::{SavingThrow, Skill},
};

use super::Feat;

#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum SkillFocus {
    Focus(Skill),
    Acrobatic,
    Alertness,
    Athletic,
    Bullheaded,
    Discipline,
    LuckOfHeroes,
    Negotiator,
    ResistPoison,
    SelfSufficient,
    SnakeBlood,
    Stealthy,
}

impl GetBonuses for SkillFocus {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0.0)
            .then(|| match self {
                SkillFocus::Focus(skill) => Some(vec![Bonus::new(
                    Attribute::Skill(*skill),
                    BonusType::Stacking,
                    3f32.into(),
                    Attribute::Feat(Feat::SkillFocus(*self)).into(),
                    None,
                )]),
                SkillFocus::Acrobatic => Some(vec![
                    Bonus::new(
                        Attribute::Skill(Skill::Jump),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Tumble),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::Alertness => Some(vec![
                    Bonus::new(
                        Attribute::Skill(Skill::Listen),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Spot),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::Athletic => Some(vec![
                    Bonus::new(
                        Attribute::Skill(Skill::Balance),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Swim),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::Bullheaded => Some(vec![
                    Bonus::new(
                        Attribute::SavingThrow(SavingThrow::Will),
                        BonusType::Stacking,
                        1f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Intimidate),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::Discipline => Some(vec![
                    Bonus::new(
                        Attribute::SavingThrow(SavingThrow::Will),
                        BonusType::Stacking,
                        1f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Concentration),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::LuckOfHeroes => Some(vec![Bonus::new(
                    Attribute::SavingThrow(SavingThrow::All),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(*self)).into(),
                    None,
                )]),
                SkillFocus::Negotiator => Some(vec![
                    Bonus::new(
                        Attribute::Skill(Skill::Diplomacy),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Haggle),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::ResistPoison => Some(vec![Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Poison),
                    BonusType::Stacking,
                    4f32.into(),
                    Attribute::Feat(Feat::SkillFocus(*self)).into(),
                    None,
                )]),
                SkillFocus::SelfSufficient => Some(vec![
                    Bonus::new(
                        Attribute::Skill(Skill::Heal),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::Repair),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::SnakeBlood => Some(vec![
                    Bonus::new(
                        Attribute::SavingThrow(SavingThrow::Reflex),
                        BonusType::Stacking,
                        1f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::SavingThrow(SavingThrow::Poison),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
                SkillFocus::Stealthy => Some(vec![
                    Bonus::new(
                        Attribute::Skill(Skill::Hide),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                    Bonus::new(
                        Attribute::Skill(Skill::MoveSilently),
                        BonusType::Stacking,
                        2f32.into(),
                        Attribute::Feat(Feat::SkillFocus(*self)).into(),
                        None,
                    ),
                ]),
            })
            .unwrap_or(None)
    }
}

impl Display for SkillFocus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillFocus::Focus(skill) => write!(f, "Skill Focus: {skill}"),
            SkillFocus::Acrobatic => write!(f, "Acrobatic"),
            SkillFocus::Athletic => write!(f, "Athletic"),
            SkillFocus::Bullheaded => write!(f, "Bullheaded"),
            SkillFocus::Discipline => write!(f, "Discipline"),
            SkillFocus::LuckOfHeroes => write!(f, "Luck of Heroes"),
            SkillFocus::Negotiator => write!(f, "Negotiator"),
            SkillFocus::ResistPoison => write!(f, "Resist Poison"),
            SkillFocus::SelfSufficient => write!(f, "Self Sufficient"),
            SkillFocus::SnakeBlood => write!(f, "Snake Blood"),
            SkillFocus::Stealthy => write!(f, "Stealthy"),
            SkillFocus::Alertness => write!(f, "Alertness"),
        }
    }
}
