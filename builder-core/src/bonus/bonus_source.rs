use enum_map::Enum;

use crate::{attribute::Attribute, feat::Feat};

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug, Enum)]
pub enum BonusSource {
    Attribute(Attribute),
    Feat(Feat),
    /// Indicates some unique identifier, indicated with a usize
    Unique(u8),
}

impl ToString for BonusSource {
    fn to_string(&self) -> String {
        match self {
            BonusSource::Attribute(attribute) => format!("Attribute: {}", attribute.to_string()),
            BonusSource::Feat(feat) => format!("Feat: {}", feat.to_string()),
            BonusSource::Unique(i) => format!("Unique: {}", i),
        }
    }
}
