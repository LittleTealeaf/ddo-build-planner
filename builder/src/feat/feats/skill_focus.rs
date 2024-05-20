use core::fmt;

use fmt::Display;

use itertools::chain;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::GetBonuses,
    bonus::{BonusTemplate, BonusType, Value},
    feat::{Feat, FeatRequirement, GetFeatRequirement, ToFeat},
    types::{saving_throw::SavingThrow, skill::Skill},
    val,
};

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug)]
/// Feats that show up under the "Skill Focus" Tab
pub enum SkillFocus {
    /// Provides a +3 bonus to a specific skill
    #[serde(rename = "s", alias = "Focus")]
    Focus(Skill),
    /// Provides a +2 bonus to Jump and Tumble
    #[serde(rename = "ac", alias = "Acrobatic")]
    Acrobatic,
    /// Provides a +2 bonus to Listen and Spot
    #[serde(rename = "al", alias = "Alertness")]
    Alertness,
    /// Provides a +2 bonus to Balance and Swim
    #[serde(rename = "at", alias = "Athletic")]
    Athletic,
    /// Provides a +1 bonus to Will and a +2 bonus to Intimidate
    #[serde(rename = "bu", alias = "Bullheaded")]
    Bullheaded,
    /// Provides a +1 bonus to Will and a +2 bonus to Concentration
    #[serde(rename = "di", alias = "Discipline")]
    Discipline,
    /// Provides a +1 bonus to all saves
    #[serde(rename = "lu", alias = "LuckOfHeroes")]
    LuckOfHeroes,
    /// Provides a +2 bonus to diplomacy and haggle
    #[serde(rename = "ne", alias = "Negotiator")]
    Negotiator,
    /// Provides a +4 bonus to saves against poison
    #[serde(rename = "re", alias = "ResistPoison")]
    ResistPoison,
    /// Provides a +2 bonus to Heal and Repair
    #[serde(rename = "se", alias = "SelfSufficient")]
    SelfSufficient,
    /// Provides a +1 bonus to Reflex and +2 bonus to saves against poison
    #[serde(rename = "sn", alias = "SnakeBlood")]
    SnakeBlood,
    /// Provides a +2 bonus to Hide and Move Silently
    #[serde(rename = "st", alias = "Stelthy")]
    Stealthy,
}

impl GetBonuses for SkillFocus {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::Focus(skill) => vec![BonusTemplate::new(*skill, BonusType::Stacking, val!(3))],
            Self::Acrobatic => vec![
                BonusTemplate::new(Skill::Jump, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Tumble, BonusType::Stacking, Value::TWO),
            ],
            Self::Alertness => vec![
                BonusTemplate::new(Skill::Listen, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Spot, BonusType::Stacking, Value::TWO),
            ],
            Self::Athletic => vec![
                BonusTemplate::new(Skill::Balance, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Swim, BonusType::Stacking, Value::TWO),
            ],
            Self::Bullheaded => vec![
                BonusTemplate::new(SavingThrow::Will, BonusType::Stacking, Value::ONE),
                BonusTemplate::new(Skill::Intimidate, BonusType::Stacking, Value::TWO),
            ],
            Self::Discipline => vec![
                BonusTemplate::new(SavingThrow::Will, BonusType::Stacking, Value::ONE),
                BonusTemplate::new(Skill::Concentration, BonusType::Stacking, Value::TWO),
            ],
            Self::LuckOfHeroes => vec![BonusTemplate::new(
                SavingThrow::All,
                BonusType::Stacking,
                Value::TWO,
            )],
            Self::Negotiator => vec![
                BonusTemplate::new(Skill::Diplomacy, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Haggle, BonusType::Stacking, Value::TWO),
            ],
            Self::ResistPoison => vec![BonusTemplate::new(
                SavingThrow::Poison,
                BonusType::Stacking,
                val!(4),
            )],
            Self::SelfSufficient => vec![
                BonusTemplate::new(Skill::Heal, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Repair, BonusType::Stacking, Value::TWO),
            ],
            Self::SnakeBlood => vec![
                BonusTemplate::new(SavingThrow::Reflex, BonusType::Stacking, Value::ONE),
                BonusTemplate::new(SavingThrow::Poison, BonusType::Stacking, Value::TWO),
            ],
            Self::Stealthy => vec![
                BonusTemplate::new(Skill::Hide, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::MoveSilently, BonusType::Stacking, Value::TWO),
            ],
        })
    }
}

impl Display for SkillFocus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
