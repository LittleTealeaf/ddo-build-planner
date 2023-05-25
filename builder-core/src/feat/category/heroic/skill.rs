use enum_map::Enum;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::{
        sub::{SavingThrow, Skill},
        Attribute,
    },
    bonus::{Bonus, BonusType, GetBonuses},
    feat::{Feat, FeatTrait},
};

/// Any heroic feats that give bonuses to skills.
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize)]
pub enum SkillFeat {
    /// Provides a +2 to [Jump](Skill::Jump) and [Tumble](Skill::Tumble)
    Acrobatic,
    /// Provides a +2 to [Listen](Skill::Listen) and [Spot](Skill::Spot)
    Alertness,
    /// Provides a +2 to [Balance](Skill::Balance) and [Swim](Skill::Swim)
    Athletic,
    /// Provides a +4 to [Concentration](Skill::Concentration)
    CombatCasting,
    /// Provides a +2 to [Diplomacy](Skill::Diplomacy) and [Haggle](Skill::Haggle)
    Negotiator,
    /// Provides a +2 to [Disable Device](Skill::DisableDevice) and [Open Lock](Skill::OpenLock)
    NimbleFingers,
    /// Provides a +2 to [Heal](Skill::Heal) and [Repair](Skill::Repair)
    SelfSufficient,
    /// Provides a +3 to any skill
    SkillFocus(Skill),
    /// Provides a +2 to [Hide](Skill::Hide) and [Move Silently](Skill::MoveSilently)
    Stealthy,
    /// Provides a +2 to [Intimidate](Skill::Intimidate) and +1 to [Will Saves](SavingThrow::Will)
    Bullheaded,
    /// Provides a +2 to [Concentration](Skill::Concentration) and +1 to [Will Saves](SavingThrow::Will)
    Discipline,
}

impl ToString for SkillFeat {
    fn to_string(&self) -> String {
        match self {
            SkillFeat::Acrobatic => String::from("Acrobatic"),
            SkillFeat::Alertness => String::from("Alertness"),
            SkillFeat::Athletic => String::from("Athletic"),
            SkillFeat::CombatCasting => String::from("Combat Casting"),
            SkillFeat::Negotiator => String::from("Negotiator"),
            SkillFeat::NimbleFingers => String::from("Nimble Fingers"),
            SkillFeat::SelfSufficient => String::from("Self Sufficient"),
            SkillFeat::SkillFocus(skill) => format!("Skill Focus: {}", skill.to_string()),
            SkillFeat::Stealthy => String::from("Stealthy"),
            SkillFeat::Bullheaded => String::from("Bullheaded"),
            SkillFeat::Discipline => String::from("Discipline"),
        }
    }
}

impl GetBonuses for SkillFeat {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            SkillFeat::Acrobatic => vec![
                Bonus::new(
                    Skill::Jump.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Acrobatic.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Tumble.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Acrobatic.into()).into(),
                    None,
                ),
            ],
            SkillFeat::Alertness => vec![
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Alertness.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Alertness.into()).into(),
                    None,
                ),
            ],
            SkillFeat::Athletic => vec![
                Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Athletic.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Swim.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Athletic.into()).into(),
                    None,
                ),
            ],
            SkillFeat::CombatCasting => vec![Bonus::new(
                Skill::Concentration.into(),
                BonusType::Stacking,
                4f32,
                Attribute::Feat(SkillFeat::CombatCasting.into()).into(),
                None,
            )],
            SkillFeat::Negotiator => vec![
                Bonus::new(
                    Skill::Diplomacy.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Negotiator.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Haggle.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Negotiator.into()).into(),
                    None,
                ),
            ],
            SkillFeat::NimbleFingers => vec![
                Bonus::new(
                    Skill::DisableDevice.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::NimbleFingers.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::OpenLock.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::NimbleFingers.into()).into(),
                    None,
                ),
            ],
            SkillFeat::SelfSufficient => vec![
                Bonus::new(
                    Skill::Heal.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::SelfSufficient.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Repair.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::SelfSufficient.into()).into(),
                    None,
                ),
            ],
            SkillFeat::SkillFocus(skill) => vec![Bonus::new(
                (*skill).into(),
                BonusType::Stacking,
                3f32,
                Attribute::Feat(SkillFeat::SkillFocus(*skill).into()).into(),
                None,
            )],
            SkillFeat::Stealthy => vec![
                Bonus::new(
                    Skill::Hide.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Stealthy.into()).into(),
                    None,
                ),
                Bonus::new(
                    Skill::MoveSilently.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Stealthy.into()).into(),
                    None,
                ),
            ],
            SkillFeat::Bullheaded => vec![
                Bonus::new(
                    Skill::Intimidate.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Bullheaded.into()).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Will.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Bullheaded.into()).into(),
                    None,
                ),
            ],
            SkillFeat::Discipline => vec![
                Bonus::new(
                    Skill::Concentration.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Discipline.into()).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Will.into(),
                    BonusType::Stacking,
                    2f32,
                    Attribute::Feat(SkillFeat::Discipline.into()).into(),
                    None,
                ),
            ],
        })
    }
}

impl FeatTrait for SkillFeat {
    fn get_description(&self) -> String {
        todo!()
    }
}

impl From<Skill> for SkillFeat {
    fn from(value: Skill) -> Self {
        Self::SkillFocus(value)
    }
}

impl From<SkillFeat> for Feat {
    fn from(value: SkillFeat) -> Self {
        Self::SkillFeat(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::bonus::BonusSource;

    use super::*;

    fn all_feats() -> Vec<SkillFeat> {
        let mut vec = vec![
            SkillFeat::Acrobatic,
            SkillFeat::Alertness,
            SkillFeat::Athletic,
            SkillFeat::CombatCasting,
            SkillFeat::Negotiator,
            SkillFeat::NimbleFingers,
            SkillFeat::SelfSufficient,
            SkillFeat::Stealthy,
            SkillFeat::Bullheaded,
            SkillFeat::Discipline,
        ];

        for skill in Skill::VALUES {
            vec.push(SkillFeat::SkillFocus(skill));
        }

        vec
    }

    #[test]
    fn zero_value_has_no_bonuses() {
        for feat in all_feats() {
            let bonuses = feat.get_bonuses(0f32);
            assert_eq!(None, bonuses);
        }
    }

    #[test]
    fn all_bonuses_have_correct_source() {
        for feat in all_feats() {
            let bonuses = feat.get_bonuses(1f32);
            assert_ne!(None, bonuses, "Feat {} grants no bonuses", feat.to_string());

            if let Some(bonuses) = bonuses {
                for bonus in bonuses {
                    assert_eq!(
                        BonusSource::Attribute(Attribute::Feat(Feat::SkillFeat(feat))),
                        bonus.get_source()
                    );
                }
            }
        }
    }
}
