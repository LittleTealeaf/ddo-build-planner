use crate::build::feat::{epic::EpicFeat, heroic::HeroicFeat};

pub struct CharacterFeats {
    Level1: Option<HeroicFeat>,
    Level3: Option<HeroicFeat>,
    Level6: Option<HeroicFeat>,
    Level9: Option<HeroicFeat>,
    Level12: Option<HeroicFeat>,
    Level15: Option<HeroicFeat>,
    Level18: Option<HeroicFeat>,
    Level21: Option<EpicFeat>,
    Level24: Option<EpicFeat>,
    Level27: Option<EpicFeat>,
    Level30: Option<EpicFeat>,
}
