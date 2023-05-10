use super::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Bonuses},
};

mod feat_trait;
pub use feat_trait::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Feat {
    Test,
}

impl ToString for Feat {
    fn to_string(&self) -> String {
        String::from(match self {
            Feat::Test => "Hi",
        })
    }
}

impl Bonuses for Feat {
    fn get_bonuses(&self) -> Vec<super::bonus::Bonus> {
        vec![Bonus::new(
            Attribute::Feat(*self),
            BonusType::Feat,
            1f32,
            BonusSource::Feat(*self),
            None,
        )]
    }
}

impl From<Feat> for Attribute {
    fn from(value: Feat) -> Self {
        Attribute::Feat(value)
    }
}

impl From<Feat> for BonusSource {
    fn from(value: Feat) -> Self {
        BonusSource::Feat(value)
    }
}
