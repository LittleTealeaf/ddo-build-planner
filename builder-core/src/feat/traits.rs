use crate::bonus::GetBonuses;

/// Requires methods that must be required by all feats.
pub trait FeatTrait: GetBonuses + ToString {
    /// Returns the description of the feat.
    fn get_description(&self) -> String;
}
