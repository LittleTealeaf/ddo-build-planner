use super::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Bonuses},
};

mod feat_trait;
pub use feat_trait::*;
mod category;
pub use category::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Feat {
    SkillFocus(SkillFocus),
    Tome(Tome),
}

impl Feat {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Feat::SkillFocus(skill_focus) => Some(skill_focus.get_feat_bonuses(value)),
            Feat::Tome(tome) => Some(tome.get_feat_bonuses(value)),
        }
    }
}

impl ToString for Feat {
    fn to_string(&self) -> String {
        match self {
            Feat::SkillFocus(skill_focus) => skill_focus.to_string(),
            Feat::Tome(tome) => tome.to_string(),
        }
    }
}

impl Bonuses for Feat {
    fn get_bonuses(&self) -> Vec<super::bonus::Bonus> {
        vec![Bonus::new(
            Attribute::Feat(*self),
            BonusType::Stacking,
            1f32,
            BonusSource::Feat(*self),
            None,
        )]
    }
    fn remove_bonuses(&self) -> Vec<Bonus> {
        vec![Bonus::new(
            Attribute::Feat(*self),
            BonusType::Stacking,
            0f32,
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
