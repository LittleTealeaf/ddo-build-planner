use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
