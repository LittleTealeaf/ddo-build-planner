mod skill;
pub use skill::*;

use super::{Feat, ToFeat};

#[derive(Clone, Copy)]
pub enum HeroicFeat {
    SkillFeat(SkillFeat),
}

impl ToFeat for HeroicFeat {
    fn to_feat(self) -> Feat {
        Feat::Heroic(self)
    }
}
