use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType},
    types::{SavingThrow, Skill}, feat::{Feat, GetFeatRequirement, FeatRequirement},
};

#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug)]
/// Feats that show up under the "Skill Focus" Tab
pub enum SkillFocus {
    /// Proides a +3 bonus to a specifc skill
    Focus(Skill),
    /// Provides a +2 bonus to Jump and Tumble
    Acrobatic,
    /// Provides a +2 bonus to Listen and Spot
    Alertness,
    /// Provides a +2 bonus to Balance and Swim
    Athletic,
    /// Provides a +1 bonus to Will and a +2 bonus to Intimidate
    Bullheaded,
    /// Provides a +1 bonus to Will and a +2 bonus to Concentration
    Discipline,
    /// Provides a +1 bonus to all saves
    LuckOfHeroes,
    /// Provides a +2 bonus to diplomacy and haggle
    Negotiator,
    /// Provides a +4 bonus to saves against poison
    ResistPoison,
    /// Provides a +2 bonus to Heal and Repair
    SelfSufficient,
    /// Provides a +1 bonus to Reflex and +2 bonus to saves against poison
    SnakeBlood,
    /// Provides a +2 bonus to Hide and Move Silently
    Stealthy,
}

impl GetBonuses for SkillFocus {
    fn get_bonuses(&self, value: f32) -> Option<Vec<crate::bonus::Bonus>> {
        (value > 0.0).then(|| match self {
            Self::Focus(skill) => vec![Bonus::new(
                Attribute::Skill(*skill),
                BonusType::Stacking,
                3f32.into(),
                Attribute::Feat(Feat::SkillFocus(Self::Focus(*skill))).into(),
                None,
            )],
            Self::Acrobatic => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Jump),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Acrobatic)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Tumble),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Acrobatic)).into(),
                    None,
                ),
            ],
            Self::Alertness => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Listen),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Alertness)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Spot),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Alertness)).into(),
                    None,
                ),
            ],
            Self::Athletic => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Balance),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Athletic)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Swim),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Athletic)).into(),
                    None,
                ),
            ],
            Self::Bullheaded => vec![
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Will),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Bullheaded)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Intimidate),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Bullheaded)).into(),
                    None,
                ),
            ],
            Self::Discipline => vec![
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Will),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Discipline)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Concentration),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Discipline)).into(),
                    None,
                ),
            ],
            Self::LuckOfHeroes => vec![Bonus::new(
                Attribute::SavingThrow(SavingThrow::All),
                BonusType::Stacking,
                2f32.into(),
                Attribute::Feat(Feat::SkillFocus(Self::LuckOfHeroes)).into(),
                None,
            )],
            Self::Negotiator => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Diplomacy),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Negotiator)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Haggle),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Negotiator)).into(),
                    None,
                ),
            ],
            Self::ResistPoison => vec![Bonus::new(
                Attribute::SavingThrow(SavingThrow::Poison),
                BonusType::Stacking,
                4f32.into(),
                Attribute::Feat(Feat::SkillFocus(Self::ResistPoison)).into(),
                None,
            )],
            Self::SelfSufficient => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Heal),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::SelfSufficient)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Repair),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::SelfSufficient)).into(),
                    None,
                ),
            ],
            Self::SnakeBlood => vec![
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Reflex),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::SnakeBlood)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Poison),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::SnakeBlood)).into(),
                    None,
                ),
            ],
            Self::Stealthy => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Hide),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Stealthy)).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::MoveSilently),
                    BonusType::Stacking,
                    2f32.into(),
                    Attribute::Feat(Feat::SkillFocus(Self::Stealthy)).into(),
                    None,
                ),
            ],
        })
    }
}

impl Display for SkillFocus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Focus(skill) => write!(f, "Skill Focus: {skill}"),
            Self::Acrobatic => write!(f, "Acrobatic"),
            Self::Athletic => write!(f, "Athletic"),
            Self::Bullheaded => write!(f, "Bullheaded"),
            Self::Discipline => write!(f, "Discipline"),
            Self::LuckOfHeroes => write!(f, "Luck of Heroes"),
            Self::Negotiator => write!(f, "Negotiator"),
            Self::ResistPoison => write!(f, "Resist Poison"),
            Self::SelfSufficient => write!(f, "Self Sufficient"),
            Self::SnakeBlood => write!(f, "Snake Blood"),
            Self::Stealthy => write!(f, "Stealthy"),
            Self::Alertness => write!(f, "Alertness"),
        }
    }
}

impl GetFeatRequirement for SkillFocus {
    fn get_feat_requirements(&self) -> Option<FeatRequirement> {
        None
    }
}
