use super::Feat;


/// Indicates that this object is a Feat
pub trait ToFeat {
    /// Converts this object into a feat
    fn to_feat(self) -> Feat;
}
