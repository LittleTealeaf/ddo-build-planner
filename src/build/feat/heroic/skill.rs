use crate::build::{
    attribute::{saving_throw::SavingThrow, skill::Skill, Attribute},
    bonus::{bonuses::Bonuses, source::Source, types::BonusType, Bonus},
    feat::Feat,
};

use super::HeroicFeat;

#[derive(Clone, Copy)]
pub enum SkillFeat {
    SkillFocus(Skill),
    Acrobatic,
    Alertness,
    Athletic,
    Bullheaded,
    Discipline,
    LuckOfTheHeroes,
    Negotiator,
    ResistPoison,
    SelfSufficient,
    SnakeBlood,
    Stealthy,
}

impl From<SkillFeat> for Feat {
    fn from(value: SkillFeat) -> Self {
        HeroicFeat::Skill(value).into()
    }
}

impl Bonuses for SkillFeat {
    fn get_bonuses(&self) -> Vec<crate::build::bonus::Bonus> {
        let source = || Source::Feat(self.clone().into());
        match self {
            SkillFeat::SkillFocus(skill) => vec![Bonus::new(
                Attribute::Skill(skill.clone()),
                BonusType::Stacking,
                3.0,
                source(),
                None,
            )],
            SkillFeat::Acrobatic => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Jump),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Tumble),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::Alertness => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Listen),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Spot),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::Athletic => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Balance),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Swim),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::Bullheaded => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Intimidate),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Will),
                    BonusType::Stacking,
                    1.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::Discipline => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Concentration),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Will),
                    BonusType::Stacking,
                    1.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::LuckOfTheHeroes => vec![
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Will),
                    BonusType::Stacking,
                    1.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Fortitude),
                    BonusType::Stacking,
                    1.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Reflex),
                    BonusType::Stacking,
                    1.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::Negotiator => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Diplomacy),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Haggle),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::ResistPoison => vec![Bonus::new(
                Attribute::SavingThrow(SavingThrow::Poison),
                BonusType::Stacking,
                4.0,
                source(),
                None,
            )],
            SkillFeat::SelfSufficient => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Heal),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Repair),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::SnakeBlood => vec![
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Reflex),
                    BonusType::Stacking,
                    1.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::SavingThrow(SavingThrow::Poison),
                    BonusType::Stacking,
                    4.0,
                    source(),
                    None,
                ),
            ],
            SkillFeat::Stealthy => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Hide),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::MoveSilently),
                    BonusType::Stacking,
                    2.0,
                    source(),
                    None,
                ),
            ],
        }
    }
}
