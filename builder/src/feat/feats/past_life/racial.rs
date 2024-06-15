use core::fmt::{self, Display};

use rust_decimal::prelude::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::GetBonuses,
    bonus::{BonusTemplate, BonusType},
    feat::{Feat, ToFeat},
    types::{ability::Ability, race::Race, skill::Skill},
};

use super::PastLifeFeat;

/// Racial Past Life Feats
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RacialPastLife(Race);

impl RacialPastLife {
    /// All racial past lives
    /// This does include races that are 'aliases' for other races, such as Wood Elf, which
    /// simply will just add bonuses to the base race
    pub const RACES: [Self; 16] = [
        Self(Race::Aasimar),
        Self(Race::Dragonborn),
        Self(Race::Drow),
        Self(Race::Dwarf),
        Self(Race::Elf),
        Self(Race::WoodElf),
        Self(Race::Gnome),
        Self(Race::HalfElf),
        Self(Race::HalfOrc),
        Self(Race::Halfling),
        Self(Race::Human),
        Self(Race::Shifter),
        Self(Race::Tabaxi),
        Self(Race::Tiefling),
        Self(Race::Warforged),
        Self(Race::Eladrin),
    ];

    /// Converts back to the race
    #[must_use]
    pub const fn get_race(&self) -> Race {
        self.0
    }

    /// Provides the base race if there are any
    #[must_use]
    pub const fn get_base(&self) -> Option<Self> {
        let Self(race) = self;
        match race {
            Race::WoodElf => Some(Self(Race::Elf)),
            _ => None,
        }
    }

    /// Returns the skill that the first racial past life gives, if any
    #[must_use]
    pub const fn get_skill(&self) -> Option<Skill> {
        let Self(race) = self;
        match race {
            Race::Aasimar => Some(Skill::Heal),
            Race::Dragonborn | Race::Tiefling => Some(Skill::Spellcraft),
            Race::Drow => Some(Skill::Search),
            Race::Dwarf => Some(Skill::Balance),
            Race::Elf | Race::Shifter => Some(Skill::Spot),
            Race::Gnome => Some(Skill::UseMagicalDevice),
            Race::HalfElf => Some(Skill::Diplomacy),
            Race::HalfOrc => Some(Skill::Intimidate),
            Race::Halfling => Some(Skill::MoveSilently),
            Race::Human => Some(Skill::Haggle),
            Race::Tabaxi => Some(Skill::Tumble),
            Race::Warforged => Some(Skill::Repair),
            Race::Eladrin => Some(Skill::Listen),
            _ => None,
        }
    }

    /// Returns the ability that the second racial past life gives, if any
    #[must_use]
    pub const fn get_ability(&self) -> Option<Ability> {
        let Self(race) = self;
        match race {
            Race::Aasimar | Race::Human => Some(Ability::Wisdom),
            Race::Dragonborn | Race::HalfElf | Race::Tiefling => Some(Ability::Charisma),
            Race::Drow | Race::Gnome => Some(Ability::Intelligence),
            Race::Dwarf | Race::Warforged => Some(Ability::Constitution),
            Race::Elf | Race::Halfling | Race::Shifter | Race::Tabaxi | Race::Eladrin => {
                Some(Ability::Dexterity)
            }
            Race::HalfOrc => Some(Ability::Strength),
            _ => None,
        }
    }
}

impl Display for RacialPastLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(race) = self;
        write!(f, "{race}")
    }
}

impl StaticOptions for RacialPastLife {
    fn get_static() -> impl Iterator<Item = Self> {
        Self::RACES.into_iter()
    }
}

impl GetBonuses for RacialPastLife {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        if value <= Decimal::ZERO {
            return None;
        }

        if let Some(race) = self.get_base() {
            return Some(vec![BonusTemplate::new(race, BonusType::Stacking, value)]);
        }

        let skill = self.get_skill()?;
        let ability = self.get_ability()?;

        Some(
            [
                (value >= dec!(1)).then(|| BonusTemplate::new(skill, BonusType::Stacking, 1)),
                (value >= dec!(2)).then(|| BonusTemplate::new(ability, BonusType::Stacking, 1)),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }
}

impl ToFeat for RacialPastLife {
    fn to_feat(self) -> Feat {
        PastLifeFeat::Racial(self).to_feat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_races_have_skill() {
        let races = RacialPastLife::RACES;
        for race in races {
            if race.get_base().is_some() {
                continue;
            }

            assert!(race.get_skill().is_some(), "Race has no skill: {race}");
        }
    }

    #[test]
    fn valid_races_have_ability() {
        let races = RacialPastLife::RACES;
        for race in races {
            if race.get_base().is_some() {
                continue;
            }
            assert!(race.get_ability().is_some(), "Race has no ability: {race}");
        }
    }
}
