use rust_decimal::prelude::Decimal;
use utils::enums::StaticOptions;

use crate::{attribute::GetBonuses, bonus::BonusTemplate, types::race::Race};

use self::racial::racial_past_lives;

mod racial;

/// Feats for past lives
pub enum PastLife {
    /// Racial Past Life Feat
    Racial(Race),
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
