use crate::build::attribute::spell::SpellSchool;

use self::{skill::SkillFeat, proficiency::ProficiencyFeat};

use super::Feat;

pub mod skill;
pub mod proficiency;

#[derive(Clone, Copy)]
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
