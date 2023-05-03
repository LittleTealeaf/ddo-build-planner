use crate::build::bonus::{bonuses::Bonuses, Bonus};

use super::{heroic::HeroicFeat, Feat};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EpicFeat {
    Heroic(HeroicFeat),
}

impl From<EpicFeat> for Feat {
    fn from(value: EpicFeat) -> Self {
        Feat::Epic(value)
    }
}

impl Bonuses for EpicFeat {
    fn get_bonuses(&self) -> Vec<Bonus> {
        match self {
            _ => Vec::new()
        }
    }
}
