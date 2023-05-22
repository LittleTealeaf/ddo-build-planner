use enum_map::Enum;

use crate::{attribute::Attribute, feat::Feat};

type UNIQUE = u8;

/// Describes the source that a bonus is from.
///
/// Sources can be either from an [Attribute](crate::bonus::BonusSource::Attribute(crate::attribute::Attribute)), [Feat](crate::bonus::BonusSource::Feat(crate::feat::Feat)), many others to come, and lastly [Unique](crate::bonus::BonusSource::Unique(u8))
#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug, Enum)]
pub enum BonusSource {
    /// Represents any source from an [Attribute].
    ///
    /// This is the most common source, as it envelops Feats and more through attributes like [Attribute::Feat]
    Attribute(Attribute),
    /// Represents sources from a [Feat]
    Feat(Feat),
    /// Indicates some unique identifier, indicated with a [u8]
    Unique(UNIQUE),
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

impl From<UNIQUE> for BonusSource {
    fn from(value: UNIQUE) -> Self {
        Self::Unique(value)
    }
}
