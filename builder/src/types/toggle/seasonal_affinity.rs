use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

use crate::types::toggle_group::ToggleGroup;

use super::{GetToggleGroup, ToToggle, Toggle};

/// Seasonal Affinity used for [`Race::Eladrin`] and [`Race::Chaosmancer`]
///
/// [`Race::Eladrin`]: crate::types::race::Race::Eladrin
/// [`Race::Chaosmancer`]: crate::types::race::Race::Chaosmancer
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SeasonalAffinity {
    /// Spring, +1 Charisma
    Spring,
    /// Summer, +1 Strength
    Summer,
    /// Autumn, +1 Wisdom
    Autumn,
    /// Winter, +1 Intelligence
    Winter,
}

impl SeasonalAffinity {
    /// All Values
    pub const ALL: [Self; 4] = [Self::Spring, Self::Summer, Self::Autumn, Self::Winter];
}

impl Display for SeasonalAffinity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spring => write!(f, "Spring"),
            Self::Summer => write!(f, "Summer"),
            Self::Autumn => write!(f, "Autumn"),
            Self::Winter => write!(f, "Winter"),
        }
    }
}

impl ToToggle for SeasonalAffinity {
    fn to_toggle(self) -> Toggle {
        Toggle::SeasonalAffinity(self)
    }
}

impl GetToggleGroup for SeasonalAffinity {
    fn custom_toggle_group(&self) -> Option<ToggleGroup> {
        Some(ToggleGroup::SeasonalAffinity)
    }
}

impl StaticValues for SeasonalAffinity {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}
