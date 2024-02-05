use core::fmt::Display;
use std::fmt;

use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::GetBonuses,
    bonus::BonusTemplate,
    feat::{Feat, ToFeat},
    types::race::Race,
};

use self::racial::racial_past_lives;

mod racial;

/// Feats for past lives
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Debug, PartialOrd, Ord)]
pub enum PastLife {
    /// Racial Past Life Feat
    Racial(Race),
}

impl Display for PastLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Racial(race) => write!(f, "Past Life: {race}"),
        }
    }
}

impl GetBonuses for PastLife {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::Racial(race) => racial_past_lives(*race, value),
        }
    }
}

impl StaticOptions for PastLife {
    fn get_static() -> impl Iterator<Item = Self> {
        Race::get_static().map(Self::Racial)
    }
}

impl ToFeat for PastLife {
    fn to_feat(self) -> Feat {
        Feat::PastLife(self)
    }
}
