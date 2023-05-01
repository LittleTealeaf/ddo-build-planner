use crate::core::{
    attribute::{Attribute, Skill},
    effects::{Bonus, BonusType, Effects},
};

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

impl Effects for SkillFeat {
    fn get_bonuses(&self) -> Vec<crate::core::effects::Bonus> {
        match self {
            Self::SkillFocus(skill) => vec![Bonus {
                attribute: Attribute::Skill(skill.clone()),
                value: 3.0,
                condition: None,
                bonus_type: BonusType::Stacking,
            }],
            Self::Acrobatic => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::Jump),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::Tumble),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
            Self::Alertness => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::Listen),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::Spot),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
            Self::Athletic => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::Balance),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::Swim),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
            Self::CombatCasting => todo!(),
            Self::Negotiator => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::Balance),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::Haggle),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
            Self::NimbleFingers => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::DisableDevice),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::OpenLock),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
            Self::SelfSufficient => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::Heal),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::Repair),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
            Self::Stealthy => vec![
                Bonus {
                    attribute: Attribute::Skill(Skill::Hide),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
                Bonus {
                    attribute: Attribute::Skill(Skill::MoveSilently),
                    value: 2.0,
                    condition: None,
                    bonus_type: BonusType::Stacking,
                },
            ],
        }
    }
}
