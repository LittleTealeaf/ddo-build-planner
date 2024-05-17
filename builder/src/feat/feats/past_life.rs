use core::{
    fmt::{self, Display},
    iter::once,
};

use itertools::chain;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};
use utils::{enums::StaticOptions, public_modules};

use crate::{
    attribute::GetBonuses,
    bonus::{BonusTemplate, BonusType},
    feat::{Feat, ToFeat},
    types::{ability::Ability, skill::Skill},
};

public_modules!(heroic, epic, racial, iconic);

/// Past Life Feat Category
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PastLifeFeat {
    /// Iconic Past Lifes
    #[serde(rename = "i", alias = "Iconic")]
    Iconic(IconicPastLife),
    /// Heroic Completionist
    #[serde(rename = "H", alias = "HeroicCompletionist")]
    HeroicCompletionist,
    /// Heroic Past Life
    #[serde(rename = "h", alias = "Heroic")]
    Heroic(HeroicPastLife),
    /// Racial Past Life
    #[serde(rename = "r", alias = "Racial")]
    Racial(RacialPastLife),
    /// Epic Past Life
    #[serde(rename = "e", alias = "Epic")]
    Epic(EpicPastLife),
    /// Racial Completionist
    #[serde(rename = "R", alias = "RacialCompletionist")]
    RacialCompletionist,
}

impl Display for PastLifeFeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iconic(iconic) => write!(f, "{iconic}"),
            Self::HeroicCompletionist => write!(f, "Heroic Completionist"),
            Self::Heroic(class) => write!(f, "{class}"),
            Self::Racial(race) => write!(f, "{race}"),
            Self::Epic(epic) => write!(f, "{epic}"),
            Self::RacialCompletionist => write!(f, "Racial Completionist"),
        }
    }
}

impl GetBonuses for PastLifeFeat {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::RacialCompletionist | Self::HeroicCompletionist => {
                (value > Decimal::ZERO).then(|| {
                    vec![
                        BonusTemplate::new(Ability::All, BonusType::Stacking, 2, None),
                        BonusTemplate::new(Skill::All, BonusType::Stacking, 2, None),
                    ]
                })
            }
            Self::Iconic(iconic) => iconic.get_bonuses(value),
            Self::Heroic(heroic) => heroic.get_bonuses(value),
            Self::Racial(race) => race.get_bonuses(value),
            Self::Epic(epic) => epic.get_bonuses(value),
        }
    }
}

impl StaticOptions for PastLifeFeat {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            once(Self::HeroicCompletionist),
            IconicPastLife::get_static().map(Self::Iconic),
            HeroicPastLife::get_static().map(Self::Heroic),
            RacialPastLife::get_static().map(Self::Racial),
            EpicPastLife::get_static().map(Self::Epic),
        )
    }
}

impl ToFeat for PastLifeFeat {
    fn to_feat(self) -> Feat {
        Feat::PastLife(self)
    }
}
