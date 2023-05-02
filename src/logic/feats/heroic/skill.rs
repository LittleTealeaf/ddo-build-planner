use crate::logic::{
    attribute::{Attribute, Skill},
    effects::{Bonus, BonusType, Effects, Source},
    feats::{Feat, ToFeat},
};

use super::HeroicFeat;

#[derive(Clone, Copy)]
pub enum SkillFeat {
    SkillFocus(Skill),
    Acrobatic,
    Alertness,
    Athletic,
    CombatCasting,
    Negotiator,
    NimbleFingers,
    SelfSufficient,
    Stealthy,
}

impl ToFeat for SkillFeat {
    fn to_feat(self) -> Feat {
        HeroicFeat::SkillFeat(self).to_feat()
    }
}

impl Effects for SkillFeat {
    fn get_bonuses(&self) -> Vec<Bonus> {
        match self {
            Self::SkillFocus(skill) => vec![Bonus::new(
                Attribute::Skill(skill.clone()),
                BonusType::Stacking,
                3.0,
                None,
                Source::Feat(self.clone().to_feat()),
            )],
            Self::Acrobatic => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Jump),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Tumble),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
            Self::Alertness => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Listen),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Spot),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
            Self::Athletic => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Balance),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Swim),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
            Self::CombatCasting => todo!(),
            Self::Negotiator => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Balance),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Haggle),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
            Self::NimbleFingers => vec![
                Bonus::new(
                    Attribute::Skill(Skill::DisableDevice),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::OpenLock),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
            Self::SelfSufficient => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Heal),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Repair),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
            Self::Stealthy => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Hide),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
                Bonus::new(
                    Attribute::Skill(Skill::MoveSilently),
                    BonusType::Stacking,
                    2.0,
                    None,
                    Source::Feat(self.clone().to_feat()),
                ),
            ],
        }
    }
}
