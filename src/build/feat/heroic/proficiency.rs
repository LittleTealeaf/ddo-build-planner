use crate::build::{
    bonus::{bonuses::Bonuses, source::Source, Bonus},
    feat::Feat,
    items::types::{ArmorType, WeaponType},
};

use super::HeroicFeat;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum ProficiencyFeat {
    Weapon(WeaponType),
    Armor(ArmorType),
}

impl From<ProficiencyFeat> for Feat {
    fn from(value: ProficiencyFeat) -> Self {
        HeroicFeat::Proficiency(value).into()
    }
}

impl Bonuses for ProficiencyFeat {
    fn get_bonuses(&self) -> Vec<crate::build::bonus::Bonus> {
        match self {
            ProficiencyFeat::Weapon(weapon) => vec![Bonus::new(
                crate::build::attribute::Attribute::WeaponProficiency(weapon.clone()),
                crate::build::bonus::types::BonusType::Flag,
                1.0,
                Source::Feat(self.clone().into()),
                None,
            )],

            ProficiencyFeat::Armor(armor) => vec![Bonus::new(
                crate::build::attribute::Attribute::ArmorProficiency(armor.clone()),
                crate::build::bonus::types::BonusType::Flag,
                1.0,
                Source::Feat(self.clone().into()),
                None,
            )],
        }
    }
}
