use crate::logic::{attribute::Attribute, feat::Feat};

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BonusSource {
    Attribute(Attribute),
    Feat(Feat),
    /// Indicates some unique identifier, indicated with a usize
    Unique(usize),
}
