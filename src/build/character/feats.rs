use crate::build::{
    bonus::{bonuses::Bonuses, Bonus}, feat::Feat,
};

pub struct CharacterFeats {
    level_1: Option<Feat>,
    level_3: Option<Feat>,
    level_6: Option<Feat>,
    level_9: Option<Feat>,
    level_12: Option<Feat>,
    level_15: Option<Feat>,
    level_18: Option<Feat>,
    level_21: Option<Feat>,
    level_24: Option<Feat>,
    level_27: Option<Feat>,
    level_30: Option<Feat>,
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
            &self.level_21,
            &self.level_24,
            &self.level_24,
            &self.level_27,
            &self.level_30,
        ] {
            if let Some(feat) = feat {
                bonuses.append(&mut feat.get_bonuses());
            }
        }

        bonuses
    }
}
