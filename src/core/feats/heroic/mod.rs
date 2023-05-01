mod skill;
pub use skill::*;

pub enum HeroicFeat {
    SkillFeat(SkillFeat),
}
