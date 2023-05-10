#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BonusType {
    Stacking,
    Enhancement,
    Equipment,
    Insightful,
    Quality,
    Feat,
    AbilityModifier,
    Artifact,
    Legendary,
}
