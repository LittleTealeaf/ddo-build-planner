use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, SavingThrow, Skill},
    bonus::{Bonus, BonusSource, BonusType},
    feat::{Feat, FeatTrait},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, enum_map::Enum)]
pub enum SkillFocus {
    Acrobatic,
    Alertness,
    Athletic,
    Bullheaded,
    Discipline,
    LuckofHeroes,
    Negotiator,
    ResistPoison,
    SelfSufficient,
    Focus(Skill),
    SnakeBlood,
    Stealthy,
}

impl ToString for SkillFocus {
    fn to_string(&self) -> String {
        match self {
            SkillFocus::Acrobatic => String::from("Acrobatic"),
            SkillFocus::Alertness => String::from("Alertness"),
            SkillFocus::Athletic => String::from("Athletic"),
            SkillFocus::Bullheaded => String::from("Bullheaded"),
            SkillFocus::Discipline => String::from("Discipline"),
            SkillFocus::LuckofHeroes => String::from("Luck of Heroes"),
            SkillFocus::Negotiator => String::from("Negotiator"),
            SkillFocus::ResistPoison => String::from("Resist Poison"),
            SkillFocus::SelfSufficient => String::from("Self Sufficient"),
            SkillFocus::Focus(skill) => format!("Skill Focus: {}", skill.to_string()),
            SkillFocus::SnakeBlood => String::from("Snake Blood"),
            SkillFocus::Stealthy => String::from("Stealthy"),
        }
    }
}

macro_rules! skill_bonus {
    ($feat: ident, $skill: ident, $value: expr) => {
        Bonus::new(
            Attribute::Skill(Skill::$skill),
            BonusType::Stacking,
            $value,
            BonusSource::Attribute(Attribute::Feat(Feat::SkillFocus(SkillFocus::$feat))),
            None,
        )
    };
}
macro_rules! saving_throw_bonus {
    ($feat: ident, $saving_throw: ident, $value: expr) => {
        Bonus::new(
            Attribute::SavingThrow(SavingThrow::$saving_throw),
            BonusType::Stacking,
            $value,
            BonusSource::Attribute(Attribute::Feat(Feat::SkillFocus(SkillFocus::$feat))),
            None,
        )
    };
}

impl FeatTrait for SkillFocus {
    fn get_feat_bonuses(&self, value: f32) -> Vec<crate::bonus::Bonus> {
        if value > 0f32 {
            match self {
                SkillFocus::Acrobatic => vec![
                    skill_bonus!(Acrobatic, Jump, 2f32),
                    skill_bonus!(Acrobatic, Tumble, 2f32),
                ],
                SkillFocus::Alertness => vec![
                    skill_bonus!(Alertness, Listen, 2f32),
                    skill_bonus!(Alertness, Spot, 2f32),
                ],
                SkillFocus::Athletic => vec![
                    skill_bonus!(Athletic, Balance, 2f32),
                    skill_bonus!(Athletic, Swim, 2f32),
                ],
                SkillFocus::Bullheaded => vec![
                    skill_bonus!(Bullheaded, Intimidate, 2f32),
                    saving_throw_bonus!(Bullheaded, Will, 1f32),
                ],
                SkillFocus::Discipline => vec![
                    skill_bonus!(Discipline, Concentration, 2f32),
                    saving_throw_bonus!(Discipline, Will, 1f32),
                ],
                SkillFocus::LuckofHeroes => vec![
                    saving_throw_bonus!(LuckofHeroes, Will, 1f32),
                    saving_throw_bonus!(LuckofHeroes, Fortitude, 1f32),
                    saving_throw_bonus!(LuckofHeroes, Reflex, 1f32),
                ],
                SkillFocus::Negotiator => vec![
                    skill_bonus!(Negotiator, Diplomacy, 2f32),
                    skill_bonus!(Negotiator, Haggle, 2f32),
                ],
                SkillFocus::ResistPoison => vec![saving_throw_bonus!(ResistPoison, Poison, 4f32)],
                SkillFocus::SelfSufficient => vec![
                    skill_bonus!(SelfSufficient, Heal, 2f32),
                    skill_bonus!(SelfSufficient, Repair, 2f32),
                ],
                SkillFocus::Focus(skill) => vec![Bonus::new(
                    Attribute::Skill(*skill),
                    BonusType::Stacking,
                    3f32,
                    BonusSource::Attribute(Attribute::Feat(Feat::SkillFocus(SkillFocus::Focus(
                        *skill,
                    )))),
                    None,
                )],
                SkillFocus::SnakeBlood => vec![
                    saving_throw_bonus!(SnakeBlood, Reflex, 1f32),
                    saving_throw_bonus!(SnakeBlood, Poison, 2f32),
                ],
                SkillFocus::Stealthy => vec![
                    skill_bonus!(Stealthy, Hide, 2f32),
                    skill_bonus!(Stealthy, MoveSilently, 2f32),
                ],
            }
        } else {
            vec![Bonus::dummy(BonusSource::Attribute(Attribute::Feat(
                Feat::SkillFocus(*self),
            )))]
        }
    }

    fn get_description(&self) -> String {
        match self {
            SkillFocus::Acrobatic => String::from("You have excellent body awareness and coordination. You receive a +2 bonus on Jump checks and Tumble checks"),
            SkillFocus::Alertness => String::from("You have finely tuned senses. You receive a +2 bonus on Listen checks and Spot checks"),
            SkillFocus::Athletic => String::from("You have a knack for athletic endeavours. You receive a +2 bonus on Balance checks and Swim checks"),
            SkillFocus::Bullheaded => String::from("You are exceptionally headstrong and difficult to sway. You receive a +1 to Will saves and a +2 bonus on Intimidate checks"),
            SkillFocus::Discipline => String::from("You are difficult to distract with spell or blow. You receive a +1 to Will saves and a +2 bonus on Concentration checks"),
            SkillFocus::LuckofHeroes => String::from("You survive when no one expects you to come through. You receive a +1 on all saves"),
            SkillFocus::Negotiator => String::from("You have a knack for gauging and swaying attitudes. You receive a +2 bonus on Diplomacy checks and Haggle checks"),
            SkillFocus::ResistPoison => String::from("You are especially resistant to poison thanks to years of training. You receive a +4 bonus to saves against poison"),
            SkillFocus::SelfSufficient => String::from("You can take care of yourself, and frequently others too. You receive a +2 bonus on Heal checks and Repair checks"),
            SkillFocus::Focus(skill) => format!("Grants a +3 bonus to all {} skill checks", skill.to_string()),
            SkillFocus::SnakeBlood => String::from("the taint of the serrpent-like yuan-ti runs in your veins. You receive a +1 to Reflex saves and a +2 bonus to saves against poison"),
            SkillFocus::Stealthy => String::from("You are particularly good at avoiding notice. You receive a +2 bonus on Hide checks and Move Silently checks"),
        }
    }
}
