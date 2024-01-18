use std::fmt::Display;

use itertools::chain;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::GetBonuses,
    bonus::{BonusTemplate, BonusType, Value},
    feat::{Feat, FeatRequirement, GetFeatRequirement, ToFeat},
    types::{saving_throw::SavingThrow, skill::Skill},
};

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug)]
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
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::Focus(skill) => vec![BonusTemplate::new(*skill, BonusType::Stacking, 3, None)],
            Self::Acrobatic => vec![
                BonusTemplate::new(Skill::Jump, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Tumble, BonusType::Stacking, 2, None),
            ],
            Self::Alertness => vec![
                BonusTemplate::new(Skill::Listen, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Spot, BonusType::Stacking, 2, None),
            ],
            Self::Athletic => vec![
                BonusTemplate::new(Skill::Balance, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Swim, BonusType::Stacking, 2, None),
            ],
            Self::Bullheaded => vec![
                BonusTemplate::new(SavingThrow::Will, BonusType::Stacking, 1, None),
                BonusTemplate::new(Skill::Intimidate, BonusType::Stacking, 2, None),
            ],
            Self::Discipline => vec![
                BonusTemplate::new(SavingThrow::Will, BonusType::Stacking, 1, None),
                BonusTemplate::new(Skill::Concentration, BonusType::Stacking, 2, None),
            ],
            Self::LuckOfHeroes => vec![BonusTemplate::new(
                SavingThrow::All,
                BonusType::Stacking,
                2,
                None,
            )],
            Self::Negotiator => vec![
                BonusTemplate::new(Skill::Diplomacy, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Haggle, BonusType::Stacking, 2, None),
            ],
            Self::ResistPoison => vec![BonusTemplate::new(
                SavingThrow::Poison,
                BonusType::Stacking,
                Value::from(4),
                None,
            )],
            Self::SelfSufficient => vec![
                BonusTemplate::new(Skill::Heal, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Repair, BonusType::Stacking, 2, None),
            ],
            Self::SnakeBlood => vec![
                BonusTemplate::new(SavingThrow::Reflex, BonusType::Stacking, 1, None),
                BonusTemplate::new(SavingThrow::Poison, BonusType::Stacking, 2, None),
            ],
            Self::Stealthy => vec![
                BonusTemplate::new(Skill::Hide, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::MoveSilently, BonusType::Stacking, 2, None),
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

impl ToFeat for SkillFocus {
    fn to_feat(self) -> Feat {
        Feat::SkillFocus(self)
    }
}

impl StaticOptions for SkillFocus {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [
                Self::Acrobatic,
                Self::Athletic,
                Self::Bullheaded,
                Self::Discipline,
                Self::LuckOfHeroes,
                Self::Negotiator,
                Self::ResistPoison,
                Self::SelfSufficient,
                Self::SnakeBlood,
                Self::Stealthy,
                Self::Alertness
            ],
            Skill::get_static().map(Self::Focus)
        )
    }
}
