use crate::{
    attribute::{sub::ClassLore, Attribute, GetBonuses},
    bonus::{Bonus, BonusSource, BonusType},
};

use super::PlayerClass;

impl PlayerClass {
    fn get_arcane_lore(&self, value: f32) -> Option<Bonus> {
        match self {
            Self::Artificer
            | Self::Bard
            | Self::Stormsinger
            | Self::Sorcerer
            | Self::AcolyteOfTheSkin
            | Self::Wizard => Some(Bonus::new(
                Attribute::ClassLore(ClassLore::Arcane),
                BonusType::Stacking,
                value,
                BonusSource::Attribute(Attribute::ClassLevel(*self)),
                None,
            )),
            _ => None,
        }
    }

    fn get_religious_lore(&self, value: f32) -> Option<Bonus> {
        match self {
            Self::Cleric
            | Self::DarkHunter
            | Self::FavoredSoul
            | Self::Paladin
            | Self::SacredFist => Some(Bonus::new(
                Attribute::ClassLore(ClassLore::Religious),
                BonusType::Stacking,
                value,
                BonusSource::Attribute(Attribute::ClassLevel(*self)),
                None,
            )),
            Self::Bard | Self::Stormsinger => Some(Bonus::new(
                Attribute::ClassLore(ClassLore::Religious),
                BonusType::Stacking,
                ((value - 1f32) / 2f32).floor(),
                BonusSource::Attribute(Attribute::ClassLevel(*self)),
                None,
            )),
            _ => None,
        }
    }

    fn get_wilderness_lore(&self, value: f32) -> Option<Bonus> {
        match self {
            Self::Barbarian | Self::Druid | Self::Ranger | Self::DarkHunter => Some(Bonus::new(
                Attribute::ClassLore(ClassLore::Wilderness),
                BonusType::Stacking,
                value,
                BonusSource::Attribute(Attribute::ClassLevel(*self)),
                None,
            )),
            Self::Bard | Self::Stormsinger => Some(Bonus::new(
                Attribute::ClassLore(ClassLore::Wilderness),
                BonusType::Stacking,
                ((value - 1f32) / 2f32).floor(),
                BonusSource::Attribute(Attribute::ClassLevel(*self)),
                None,
            )),
            _ => None,
        }
    }
}

impl GetBonuses for PlayerClass {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        if value == 0f32 {
            return None;
        }

        let mut vec = Vec::new();

        // Class Lores
        vec.append(
            &mut [
                self.get_arcane_lore(value),
                self.get_religious_lore(value),
                self.get_wilderness_lore(value),
            ]
            .into_iter()
            .flatten()
            .collect(),
        );

        Some(vec)
    }
}
