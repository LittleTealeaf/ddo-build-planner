use crate::build::{
    bonus::{bonuses::Bonuses, Bonus},
    feat::{epic::EpicFeat, heroic::HeroicFeat},
};

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

impl Bonuses for CharacterFeats {
    fn get_bonuses(&self) -> Vec<Bonus> {
        let mut bonuses = Vec::new();
        for feat in [
            &self.level_1,
            &self.level_3,
            &self.level_6,
            &self.level_9,
            &self.level_12,
            &self.level_15,
            &self.level_18,
        ] {
            if let Some(feat) = feat {
                bonuses.append(&mut feat.get_bonuses());
            }
        }

        for feat in [
            &self.level_21,
            &self.level_24,
            &self.level_24,
            &self.level_27,
            &self.level_30,
        ] {
            if let Some(feat) = feat {
                bonuses.append(&mut feat.get_bonuses())
            }
        }
        
        bonuses
    }
}
