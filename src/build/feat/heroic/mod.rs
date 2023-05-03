use crate::build::{attribute::spell::SpellSchool, bonus::bonuses::Bonuses};

use self::{proficiency::ProficiencyFeat, skill::SkillFeat};

use super::Feat;

pub mod proficiency;
pub mod skill;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum HeroicFeat {
    SpellFocus(SpellSchool),
    GreaterSpellFocus(SpellSchool),
    Skill(SkillFeat),
    Proficiency(ProficiencyFeat),
}

impl From<HeroicFeat> for Feat {
    fn from(value: HeroicFeat) -> Self {
        Self::Heroic(value)
    }
}

impl Bonuses for HeroicFeat {
    fn get_bonuses(&self) -> Vec<crate::build::bonus::Bonus> {
        match self {
            Self::Skill(skill_feat) => skill_feat.get_bonuses(),
            _ => Vec::new(),
        }
    }
}
