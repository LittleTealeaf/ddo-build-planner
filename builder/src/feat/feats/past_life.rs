use core::fmt::{self, Display};

use itertools::chain;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utils::{enums::StaticOptions, public_modules};

use crate::{attribute::GetBonuses, bonus::BonusTemplate};

public_modules!(heroic, epic, racial, iconic);

/// Past Life Feat Category
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PastLifeFeat {
    /// Iconic Past Lifes
    Iconic(IconicPastLife),
}

impl Display for PastLifeFeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iconic(iconic) => write!(f, "{iconic}"),
        }
    }
}

impl GetBonuses for PastLifeFeat {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::Iconic(iconic) => iconic.get_bonuses(value),
        }
    }
}

impl StaticOptions for PastLifeFeat {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(IconicPastLife::get_static().map(Self::Iconic))
    }
}
