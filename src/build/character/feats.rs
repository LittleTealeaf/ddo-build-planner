use crate::build::feat::{epic::EpicFeat, heroic::HeroicFeat};

pub struct CharacterFeats {
    level_1: Option<HeroicFeat>,
    level_3: Option<HeroicFeat>,
    level_6: Option<HeroicFeat>,
    level_9: Option<HeroicFeat>,
    level_12: Option<HeroicFeat>,
    level_15: Option<HeroicFeat>,
    level_18: Option<HeroicFeat>,
    level_21: Option<EpicFeat>,
    level_24: Option<EpicFeat>,
    level_27: Option<EpicFeat>,
    level_30: Option<EpicFeat>,
}
