use core::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::types::{race::Race, toggle_group::ToggleGroup};

use super::{GetToggleGroup, ToToggle, Toggle};

/// Depicts an Iconic Past Life
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IconicPastLife(pub Race);

impl Display for IconicPastLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(race) = self;
        write!(f, "Iconic Past Life: {race}")
    }
}

impl StaticOptions for IconicPastLife {
    fn get_static() -> impl Iterator<Item = Self> {
        [
            Race::Scourge,
            Race::Bladeforged,
            Race::DeepGnome,
            Race::PurpleDragonKnight,
            Race::Razorclaw,
            Race::Shadarkai,
            Race::Morninglord,
            Race::Trailblazer,
            Race::Scoundrel,
        ]
        .into_iter()
        .map(Self)
    }
}

impl ToToggle for IconicPastLife {
    fn to_toggle(self) -> Toggle {
        Toggle::IconicPastLife(self)
    }
}

impl GetToggleGroup for IconicPastLife {
    fn toggle_group(&self) -> Option<ToggleGroup> {
        Some(ToggleGroup::IconicPastLife)
    }
}
