use serde::{Deserialize, Serialize};

use crate::{
    attribute::{
        sub::{Ability, SpellPower},
        Attribute,
    },
    bonus::{Bonus, BonusSource, BonusType},
    feat::{Feat, FeatTrait},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, enum_map::Enum)]
pub enum Tome {
    Ability(Ability),
    SpellPower,
    PhysicalSheltering,
    MagicalSheltering,
}

impl ToString for Tome {
    fn to_string(&self) -> String {
        match self {
            Self::Ability(ability) => format!("Inherent {}", ability.to_string()),
            Self::SpellPower => String::from("Inherent Spell Power"),
            Self::PhysicalSheltering => String::from("Inherent Physical Resistance"),
            Self::MagicalSheltering => String::from("Inherent Magical Resistance"),
        }
    }
}

impl FeatTrait for Tome {
    fn get_feat_bonuses(&self, value: f32) -> Vec<crate::bonus::Bonus> {
        match self {
            Tome::Ability(ability) => vec![Bonus::new(
                Attribute::Ability(*ability),
                BonusType::Stacking,
                value,
                BonusSource::Feat(Feat::Tome(Tome::Ability(*ability))),
                None,
            )],
            Tome::SpellPower => vec![Bonus::new(
                Attribute::SpellPower(SpellPower::Universal),
                BonusType::Stacking,
                value,
                BonusSource::Feat(Feat::Tome(Tome::SpellPower)),
                None,
            )],
            Tome::PhysicalSheltering => vec![Bonus::new(
                Attribute::PhysicalSheltering(),
                BonusType::Stacking,
                value,
                BonusSource::Feat(Feat::Tome(Tome::PhysicalSheltering)),
                None,
            )],
            Tome::MagicalSheltering => vec![Bonus::new(
                Attribute::MagicalSheltering(),
                BonusType::Stacking,
                value,
                BonusSource::Feat(Feat::Tome(Tome::MagicalSheltering)),
                None,
            )],
        }
    }

    fn get_description(&self) -> String {
        match self {
            Self::Ability(ability) => format!("+1 Inherent Bonus to {}", ability.to_string()),
            Self::SpellPower => String::from("+1 bonus to Spell Power"),
            Self::PhysicalSheltering => String::from("+1 bonus to Physical Sheltering"),
            Self::MagicalSheltering => String::from("+1 bonus to Magical Sheltering"),
        }
    }
}
