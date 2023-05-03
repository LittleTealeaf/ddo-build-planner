use super::{heroic::HeroicFeat, Feat};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpicFeat {
    Heroic(HeroicFeat),
}

impl From<EpicFeat> for Feat {
    fn from(value: EpicFeat) -> Self {
        Feat::Epic(value)
    }
}
